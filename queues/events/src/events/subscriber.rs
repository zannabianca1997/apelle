use ::tokio::sync::broadcast;
use apelle_common::common_errors::{PubSubError, PubSubSnafu};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use futures::stream::TryStreamExt as _;
use redis::{FromRedisValue, PushInfo, aio::AsyncPushSender};
use snafu::{ResultExt, Snafu};
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;

use crate::events::{CHANNEL_PATTERN, CHANNEL_PREFIX, Event};

#[derive(Clone, AsRef, AsMut, Deref, DerefMut)]
pub struct SubscribedClient {
    #[as_ref]
    #[as_mut]
    #[deref]
    #[deref_mut]
    pub_sub: redis::aio::ConnectionManager,

    sender: PushSender,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub capacity: usize,
    pub pub_sub: redis::aio::ConnectionManagerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            capacity: 1024,
            pub_sub: Default::default(),
        }
    }
}

impl SubscribedClient {
    pub async fn new(
        client: redis::Client,
        Config { capacity, pub_sub }: Config,
    ) -> Result<Self, PubSubError> {
        let sender = PushSender::new(capacity);

        let mut pub_sub = redis::aio::ConnectionManager::new_with_config(
            client,
            pub_sub
                .set_automatic_resubscription()
                .set_push_sender(sender.clone()),
        )
        .await
        .context(PubSubSnafu)?;

        pub_sub
            .psubscribe(CHANNEL_PATTERN)
            .await
            .context(PubSubSnafu)?;

        Ok(Self { pub_sub, sender })
    }

    pub fn events(
        &self,
        queue: Uuid,
        user: Uuid,
    ) -> impl Stream<Item = Result<json_patch::Patch, PatchesLost>> + use<> {
        BroadcastStream::new(self.sender.inner.subscribe())
            // Creating lagging lost events
            .map(move |r| {
                r.map_err(|err| match err {
                    tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n) => {
                        tracing::warn!(%queue, %user, count = n, "Lost events due to lagging behind");
                        PartialEventsLost {
                            target: None,
                            count: Some(n),
                        }
                    }
                })?
            })
            // Filtering events for the queue and user requested
            .filter(move |event| {
                let (target_queue, target_user) = match event {
                    Ok(e) => (e.queue, e.user),
                    Err(PartialEventsLost { target: Some(target), .. }) => *target,
                    Err(PartialEventsLost { target: None, .. }) => return true,
                };
                target_queue == queue && target_user.is_none_or(|u| u == user)
            })
            // Converting events to patches
            .map_ok(|Event { content, .. }| content)
            .map_err(|PartialEventsLost { target:_, count }| PatchesLost { count })
    }
}

#[derive(Debug, Clone, Copy, Snafu)]
pub struct PatchesLost {
    /// If known, the number of lost patches
    pub count: Option<u64>,
}

#[derive(Debug, Clone, Copy, Snafu)]
struct PartialEventsLost {
    /// If relevable, the target of the lost events
    target: Option<(Uuid, Option<Uuid>)>,
    /// If known, the number of lost events
    count: Option<u64>,
}

#[derive(Clone, Debug)]
struct PushSender {
    inner: broadcast::Sender<Result<Event, PartialEventsLost>>,
}
impl PushSender {
    fn new(capacity: usize) -> Self {
        Self {
            inner: broadcast::Sender::new(capacity),
        }
    }
}

impl AsyncPushSender for PushSender {
    #[tracing::instrument(skip(self))]
    fn send(&self, PushInfo { kind, data }: PushInfo) -> Result<(), redis::aio::SendError> {
        tracing::info!("Received message");
        let event = match kind {
            redis::PushKind::Message
            | redis::PushKind::SMessage
            | redis::PushKind::Unsubscribe
            | redis::PushKind::SUnsubscribe
            | redis::PushKind::Subscribe
            | redis::PushKind::SSubscribe => {
                tracing::error!("The client subscribes only via `psubscribe`");
                return Ok(());
            }
            redis::PushKind::Other(_) => {
                tracing::warn!("Unknown push kind");
                return Ok(());
            }
            redis::PushKind::Invalidate => {
                tracing::error!("No key in the keyspace should never be invalidated");
                return Ok(());
            }

            redis::PushKind::Disconnection => {
                tracing::warn!(
                    "Client disconnected. Signalling all listeners we might have lost some events"
                );
                Err(PartialEventsLost {
                    target: None,
                    count: None,
                })
            }

            redis::PushKind::PUnsubscribe => {
                tracing::error!("The client should never unsubscribe from a pattern");
                return Ok(());
            }
            redis::PushKind::PSubscribe => {
                tracing::info!("Client subscribed");
                return Ok(());
            }

            redis::PushKind::PMessage => {
                assert_eq!(data.len(), 3, "The data should be a throuple");
                let mut data = data.into_iter().skip(1).map(String::from_owned_redis_value);

                let channel = data
                    .next()
                    .unwrap()
                    .expect("The first element should be the key");
                let payload = data
                    .next()
                    .unwrap()
                    .expect("The payload should be a string");

                let (queue, user) = {
                    let qu = channel.strip_prefix(CHANNEL_PREFIX).unwrap();
                    qu.split_once(':').map_or((qu, None), |(q, u)| (q, Some(u)))
                };

                let queue = Uuid::parse_str(queue).unwrap();
                let user = user.map(|u| Uuid::parse_str(u).unwrap());

                let content =
                    serde_json::from_str(&payload).expect("All the payloads should be valid json");

                Ok(Event {
                    queue,
                    user,
                    content,
                })
            }
        };

        // We do not care if no one is listening
        self.inner.send(event).ok();

        Ok(())
    }
}

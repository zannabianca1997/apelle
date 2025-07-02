use std::io::Write;

use apelle_common::common_errors::{PubSubError, PubSubSnafu};
use axum::extract::FromRef;
use redis::AsyncCommands as _;
use snafu::ResultExt as _;

use crate::events::CHANNEL_PREFIX;

use super::Event;

pub struct Publisher {
    client: redis::aio::ConnectionManager,
}

impl Publisher {
    pub fn new(client: redis::aio::ConnectionManager) -> Self {
        Self { client }
    }

    pub async fn publish(
        &mut self,
        Event {
            queue,
            user,
            content,
        }: Event,
    ) -> Result<(), PubSubError> {
        let mut buffer = [0u8; CHANNEL_PREFIX.len() + 36 + 1 + 36];
        let channel = {
            let mut unwritten = &mut buffer[..];

            write!(unwritten, "{}{}", CHANNEL_PREFIX, queue).unwrap();
            if let Some(user) = user {
                write!(unwritten, ":{}", user).unwrap();
            }

            let unwritten = unwritten.len();
            let written = buffer.len() - unwritten;
            str::from_utf8(&buffer[..written]).unwrap()
        };

        self.client
            .publish(channel, serde_json::to_string(&content).unwrap())
            .await
            .context(PubSubSnafu)
    }
}

impl Event {
    pub async fn publish(self, publisher: &mut Publisher) -> Result<(), PubSubError> {
        publisher.publish(self).await
    }
}

impl<S> FromRef<S> for Publisher
where
    redis::aio::ConnectionManager: FromRef<S>,
{
    fn from_ref(s: &S) -> Self {
        Self::new(redis::aio::ConnectionManager::from_ref(s))
    }
}

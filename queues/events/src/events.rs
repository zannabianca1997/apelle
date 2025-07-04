use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod builder;
mod collector;
mod publisher;
mod subscriber;

const CHANNEL_PREFIX: &str = "apelle:queues:events:";
const CHANNEL_PATTERN: &str = "apelle:queues:events:*";

pub use builder::{BuildPatchEvent, PatchEventBuilder, QueueEventBuilder, UserEventBuilder};
pub use collector::{Collector, event_middleware};
pub use publisher::Publisher;
pub use subscriber::{Config as SubscribedClientConfig, PatchesLost, SubscribedClient};

#[derive(Debug, Clone)]
pub struct Event {
    queue: Uuid,
    user: Option<Uuid>,
    content: EventContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum EventContent {
    /// The queue was deleted
    Deleted,
    /// A patch to apply to the queue data
    Patch(json_patch::Patch),
    /// The entire value of the queue data
    Sync(serde_json::Value),
}

impl Event {
    /// Create an event for all viewers of a queue
    pub fn queue(queue: Uuid) -> builder::QueueEventBuilder {
        builder::QueueEventBuilder::new(queue)
    }

    /// Create an event for a specific user of a queue
    pub fn user(queue: Uuid, user: Uuid) -> builder::UserEventBuilder {
        builder::UserEventBuilder::new(queue, user)
    }

    pub fn coalesce(self, other: Self) -> Result<Self, (Self, Self)> {
        if self.queue != other.queue || self.user != other.user {
            // Not the same target
            return Err((self, other));
        }

        match EventContent::coalesce(self.content, other.content) {
            Ok(content) => Ok(Self {
                queue: self.queue,
                user: self.user,
                content,
            }),
            Err((content, other_content)) => Err((
                Self {
                    queue: self.queue,
                    user: self.user,
                    content,
                },
                Self {
                    queue: other.queue,
                    user: other.user,
                    content: other_content,
                },
            )),
        }
    }
}

impl EventContent {
    pub fn coalesce(self, other: Self) -> Result<Self, (Self, Self)> {
        match (self, other) {
            (Self::Deleted, _) | (_, Self::Deleted) => Ok(Self::Deleted),
            (Self::Patch(json_patch::Patch(mut a)), Self::Patch(json_patch::Patch(b))) => {
                Ok(Self::Patch({
                    a.extend(b);
                    json_patch::Patch(a)
                }))
            }
            (_, sync @ Self::Sync(_)) => Ok(sync),
            (Self::Sync(mut doc), Self::Patch(patch)) => {
                let Ok(()) = json_patch::patch(&mut doc, &patch) else {
                    tracing::warn!(sync =? doc, ?patch, "Patch failed to apply to preceding sync");
                    return Err((Self::Sync(doc), Self::Patch(patch)));
                };
                Ok(Self::Sync(doc))
            }
        }
    }
}

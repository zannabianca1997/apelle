use uuid::Uuid;

mod builder;
mod publisher;
mod subscriber;

const CHANNEL_PREFIX: &str = "apelle:queues:events:";
const CHANNEL_PATTERN: &str = "apelle:queues:events:*";

pub use publisher::Publisher;
pub use subscriber::SubscribedClient;

#[derive(Debug, Clone)]
pub struct Event {
    queue: Uuid,
    user: Option<Uuid>,
    content: json_patch::Patch,
}

impl Event {
    /// Create an event for all viewers of a queue
    pub fn queue(queue: Uuid) -> builder::EventBuilder {
        builder::EventBuilder {
            queue,
            user: None,
            content: json_patch::Patch::default(),
        }
    }

    /// Create an event for a specific user of a queue
    pub fn user(queue: Uuid, user: Uuid) -> builder::EventBuilder {
        builder::EventBuilder {
            queue,
            user: Some(user),
            content: json_patch::Patch::default(),
        }
    }

    pub const fn len(&self) -> usize {
        self.content.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.content.0.is_empty()
    }
}

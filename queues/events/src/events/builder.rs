use uuid::Uuid;

pub struct EventBuilder {
    pub(super) queue: Uuid,
    pub(super) user: Option<Uuid>,
    pub(super) content: json_patch::Patch,
}

impl EventBuilder {
    pub fn operation(mut self, operation: json_patch::PatchOperation) -> Self {
        self.content.0.push(operation);
        self
    }

    pub fn then(mut self, other: json_patch::Patch) -> Self {
        self.content.0.extend(other.0);
        self
    }

    pub fn build(self) -> super::Event {
        super::Event {
            queue: self.queue,
            user: self.user,
            content: self.content,
        }
    }
}

impl From<EventBuilder> for super::Event {
    fn from(builder: EventBuilder) -> Self {
        builder.build()
    }
}

use json_patch::{
    AddOperation, CopyOperation, MoveOperation, PatchOperation, RemoveOperation, ReplaceOperation,
    TestOperation, jsonptr::PointerBuf,
};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use super::{Event, EventContent};

#[must_use]
pub struct QueueEventBuilder {
    queue: Uuid,
}

impl QueueEventBuilder {
    /// Create an event for a whole queue
    pub fn new(queue: Uuid) -> Self {
        Self { queue }
    }

    /// Create a sync event
    pub fn sync(self, value: Value) -> Event {
        Event {
            queue: self.queue,
            user: None,
            content: EventContent::Sync(value),
        }
    }

    // Signal that the queue was deleted
    pub fn deleted(self) -> Event {
        Event {
            queue: self.queue,
            user: None,
            content: EventContent::Deleted,
        }
    }
}

#[must_use]
pub struct UserEventBuilder {
    queue: Uuid,
    user: Uuid,
}

impl UserEventBuilder {
    /// Create an event for a specific user of a queue
    pub fn new(queue: Uuid, user: Uuid) -> Self {
        Self { queue, user }
    }

    /// Create a sync event
    pub fn sync(self, value: impl Serialize) -> Event {
        Event {
            queue: self.queue,
            user: Some(self.user),
            content: EventContent::Sync(serde_json::to_value(value).unwrap()),
        }
    }
}

pub trait BuildPatchEvent {
    type Out: BuildPatchEvent;

    fn add(self, path: impl Into<String>, value: impl Serialize) -> Self::Out;
    fn remove(self, path: impl Into<String>) -> Self::Out;
    fn replace(self, path: impl Into<String>, value: impl Serialize) -> Self::Out;
    fn move_(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out;
    fn copy(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out;
    fn test(self, path: impl Into<String>, value: impl Serialize) -> Self::Out;
}

#[must_use]
pub struct PatchEventBuilder {
    queue: Uuid,
    user: Option<Uuid>,
    patch: json_patch::Patch,
}

impl PatchEventBuilder {
    pub fn queue(queue: Uuid) -> Self {
        Self {
            queue,
            user: None,
            patch: json_patch::Patch::default(),
        }
    }
    pub fn user(queue: Uuid, user: Uuid) -> Self {
        Self {
            queue,
            user: Some(user),
            patch: json_patch::Patch::default(),
        }
    }

    pub fn build(self) -> Event {
        Event {
            queue: self.queue,
            user: self.user,
            content: EventContent::Patch(self.patch),
        }
    }
}

impl BuildPatchEvent for QueueEventBuilder {
    type Out = PatchEventBuilder;

    fn add(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::queue(self.queue).add(path, value)
    }

    fn remove(self, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::queue(self.queue).remove(path)
    }

    fn replace(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::queue(self.queue).replace(path, value)
    }

    fn move_(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::queue(self.queue).move_(from, path)
    }

    fn copy(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::queue(self.queue).copy(from, path)
    }

    fn test(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::queue(self.queue).test(path, value)
    }
}

impl BuildPatchEvent for UserEventBuilder {
    type Out = PatchEventBuilder;

    fn add(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).add(path, value)
    }

    fn remove(self, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).remove(path)
    }

    fn replace(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).replace(path, value)
    }

    fn move_(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).move_(from, path)
    }

    fn copy(self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).copy(from, path)
    }

    fn test(self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        PatchEventBuilder::user(self.queue, self.user).test(path, value)
    }
}

impl BuildPatchEvent for PatchEventBuilder {
    type Out = Self;

    fn add(mut self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        self.patch.0.push(PatchOperation::Add(AddOperation {
            path: PointerBuf::parse(path).unwrap(),
            value: serde_json::to_value(value).unwrap(),
        }));
        self
    }

    fn remove(mut self, path: impl Into<String>) -> Self::Out {
        self.patch.0.push(PatchOperation::Remove(RemoveOperation {
            path: PointerBuf::parse(path).unwrap(),
        }));
        self
    }

    fn replace(mut self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        self.patch.0.push(PatchOperation::Replace(ReplaceOperation {
            path: PointerBuf::parse(path).unwrap(),
            value: serde_json::to_value(value).unwrap(),
        }));
        self
    }

    fn move_(mut self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        self.patch.0.push(PatchOperation::Move(MoveOperation {
            from: PointerBuf::parse(from).unwrap(),
            path: PointerBuf::parse(path).unwrap(),
        }));
        self
    }

    fn copy(mut self, from: impl Into<String>, path: impl Into<String>) -> Self::Out {
        self.patch.0.push(PatchOperation::Copy(CopyOperation {
            from: PointerBuf::parse(from).unwrap(),
            path: PointerBuf::parse(path).unwrap(),
        }));
        self
    }

    fn test(mut self, path: impl Into<String>, value: impl Serialize) -> Self::Out {
        self.patch.0.push(PatchOperation::Test(TestOperation {
            path: PointerBuf::parse(path).unwrap(),
            value: serde_json::to_value(value).unwrap(),
        }));
        self
    }
}

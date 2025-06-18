use std::collections::{HashMap, HashSet};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::QueueUserAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueUserRole {
    pub id: Uuid,

    pub max_likes: u16,

    pub permissions: HashSet<QueueUserAction>,

    pub can_grant: HashSet<String>,
    pub can_revoke: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub id: Uuid,

    pub creator_role: String,
    pub default_role: String,
    pub banned_role: String,

    pub roles: HashMap<String, QueueUserRole>,

    pub autolike: bool,

    pub created: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueUserRoleCreate {
    pub max_likes: u16,

    pub permissions: HashSet<QueueUserAction>,

    pub can_grant: HashSet<String>,
    pub can_revoke: HashSet<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigCreate {
    pub creator_role: String,
    pub default_role: String,
    pub banned_role: String,

    pub roles: HashMap<String, QueueUserRoleCreate>,

    #[serde(default = "default_autolike", skip_serializing_if = "Clone::clone")]
    pub autolike: bool,
}

fn default_autolike() -> bool {
    true
}

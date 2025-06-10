use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::QueueUserAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueUserRole {
    pub max_likes: u16,

    pub permissions: HashSet<QueueUserAction>,

    pub can_grant: HashSet<String>,
    pub can_revoke: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub creator_role: String,
    pub default_role: String,
    pub banned_role: String,

    pub roles: HashMap<String, QueueUserRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueUserRoleUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_likes: Option<u16>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<HashSet<QueueUserAction>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_grant: Option<HashSet<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_revoke: Option<HashSet<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfigUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banned_role: Option<String>,

    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub roles: HashMap<String, Option<QueueUserRoleUpdate>>,
}

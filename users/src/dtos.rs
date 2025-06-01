use std::collections::HashSet;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data to create a new user
#[derive(Debug, Clone, Deserialize)]
pub struct UserCreateDto {
    /// Unique user name
    pub name: String,
    /// Password
    pub password: String,
}

/// Data about a user
#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    /// Identifier of the user
    pub id: Uuid,
    /// Unique user name
    pub name: String,
    /// Roles of the user
    pub roles: HashSet<String>,
    /// When the user was created
    pub created: DateTime<FixedOffset>,
    /// When the user was last updated
    pub updated: DateTime<FixedOffset>,
    /// When the user was last seen
    pub last_login: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdateDto {
    pub name: Option<String>,
    pub password: Option<String>,
}

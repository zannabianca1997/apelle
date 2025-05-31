use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
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
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserDto {
    /// Identifier of the user
    pub id: Uuid,
    /// Unique user name
    pub name: String,
    /// Roles of the user
    pub roles: HashSet<String>,
    /// When the user was created
    pub created: DateTime<Utc>,
    /// When the user was last updated
    pub updated: DateTime<Utc>,
    /// When the user was last seen
    pub last_login: Option<DateTime<Utc>>,
}

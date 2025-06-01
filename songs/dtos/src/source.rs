//! Sources related DTOs

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Create a new source
///
/// This is sent from source providers to the songs service
/// to signal that a new source is available
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceRegister {
    pub urn: String,
    pub name: String,
}

/// Information about a source registered in the database
#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub id: Uuid,
    pub urn: String,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub last_heard: DateTime<FixedOffset>,
}

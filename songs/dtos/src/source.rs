//! Sources related DTOs

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Create a new source
///
/// This is sent from source providers to the songs service
/// to signal that a new source is available
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct SourceRegister {
    pub urn: String,
    pub name: String,
}

/// Like [`SourceRegister`] but as a reference
///
/// Used to serialize constants ('a = 'static)
#[derive(Debug, Clone, Serialize)]
pub struct SourceRegisterRef<'a> {
    pub urn: &'a str,
    pub name: &'a str,
}

/// Information about a source registered in the database
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Source {
    pub urn: String,
    pub name: String,
    pub created: DateTime<FixedOffset>,
    pub last_heard: DateTime<FixedOffset>,
}

use std::collections::HashMap;

use apelle_common::id_or_rep::IdOrRep;
use apelle_configs_dtos::QueueConfig;
use apelle_songs_dtos::public::Song;
use chrono::{DateTime, Duration, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::QueueUserDto;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct Queue {
    pub id: Uuid,
    pub code: String,

    pub player_state_id: Uuid,

    pub current: Option<Current>,

    pub config: IdOrRep<QueueConfig>,
    pub user: QueueUserDto,

    pub queue: HashMap<Uuid, QueuedSong>,

    pub created: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct Current {
    song: IdOrRep<Song>,
    #[serde(flatten)]
    state: TimeRef,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
#[serde(untagged)]
enum TimeRef {
    Relative {
        #[serde(with = "apelle_common::iso8601::duration")]
        position: Duration,
    },
    Absolute {
        starts_at: DateTime<FixedOffset>,
    },
}

impl Current {
    pub fn playing(song: IdOrRep<Song>, starts_at: DateTime<FixedOffset>) -> Self {
        Self {
            song,
            state: TimeRef::Absolute { starts_at },
        }
    }
    pub fn stopped(song: IdOrRep<Song>, position: Duration) -> Self {
        Self {
            song,
            state: TimeRef::Relative { position },
        }
    }

    pub fn song(&self) -> &IdOrRep<Song> {
        &self.song
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct QueuedSong {
    /// Song that was queued
    pub song: IdOrRep<Song>,
    /// When the song was queued
    pub queued_at: DateTime<FixedOffset>,
    /// Number of likes this song has
    pub likes: u16,
    /// Number of likes this song has by the current user
    pub user_likes: u16,
}

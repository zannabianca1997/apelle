use std::collections::HashMap;

use apelle_common::id_or_rep::IdOrRep;
use apelle_configs_dtos::QueueConfig;
use apelle_songs_dtos::public::Song;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Queue {
    pub id: Uuid,
    pub code: String,

    pub player_state_id: Uuid,

    pub current: Option<Current>,

    pub config: IdOrRep<QueueConfig>,

    pub queue: HashMap<Uuid, QueuedSong>,

    pub created: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Current {
    song: Song,
    #[serde(flatten)]
    state: TimeRef,
}

#[derive(Debug, Clone, Copy, Serialize, ToSchema)]
#[serde(untagged)]
enum TimeRef {
    Relative { position: Duration },
    Absolute { starts_at: DateTime<FixedOffset> },
}

impl Current {
    pub fn playing(song: Song, starts_at: DateTime<FixedOffset>) -> Self {
        Self {
            song,
            state: TimeRef::Absolute { starts_at },
        }
    }
    pub fn stopped(song: Song, position: Duration) -> Self {
        Self {
            song,
            state: TimeRef::Relative { position },
        }
    }

    pub fn song(&self) -> &Song {
        &self.song
    }

    /// Current position of the song
    pub fn position(&self) -> Duration {
        let now = Local::now();

        match self.state {
            TimeRef::Relative { position } => position,
            TimeRef::Absolute { starts_at } => now.signed_duration_since(starts_at),
        }
        .clamp(Duration::zero(), self.song.duration)
    }

    /// Point in time when the song should have started to reach the current
    /// position
    pub fn starts_at(&self) -> DateTime<FixedOffset> {
        let now = Local::now().fixed_offset();

        match self.state {
            TimeRef::Relative { position } => now - position,
            TimeRef::Absolute { starts_at } => starts_at,
        }
        .clamp(now - self.song.duration, now)
    }

    /// If the song is stopped
    pub fn is_stopped(&self) -> bool {
        match self.state {
            TimeRef::Relative { .. } => true,
            TimeRef::Absolute { starts_at } => {
                Local::now().signed_duration_since(starts_at) >= self.song.duration
            }
        }
    }

    /// If the song is playing
    pub fn is_playing(&self) -> bool {
        !self.is_stopped()
    }

    /// If the song was manually paused
    pub fn paused(&self) -> bool {
        match self.state {
            TimeRef::Relative { .. } => true,
            TimeRef::Absolute { .. } => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
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

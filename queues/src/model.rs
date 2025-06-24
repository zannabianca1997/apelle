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

    pub current: Option<Current>,

    pub config: IdOrRep<QueueConfig>,

    pub queue: HashMap<Uuid, QueuedSong>,

    pub created: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Current {
    song: Song,
    player_state_id: Uuid,
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
    pub fn playing(song: Song, player_state_id: Uuid, starts_at: DateTime<FixedOffset>) -> Self {
        Self {
            song,
            player_state_id,
            state: TimeRef::Absolute { starts_at },
        }
    }
    pub fn stopped(song: Song, player_state_id: Uuid, position: Duration) -> Self {
        Self {
            song,
            player_state_id,
            state: TimeRef::Relative { position },
        }
    }

    pub fn song(&self) -> &Song {
        &self.song
    }

    pub fn player_state_id(&self) -> Uuid {
        self.player_state_id
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

    /// Pause the song
    pub fn pause(&mut self) -> bool {
        if self.paused() {
            return false;
        }
        self.state = TimeRef::Relative {
            position: self.position(),
        };
        self.player_state_id = Uuid::new_v4();
        true
    }

    /// Resume the song
    pub fn resume(&mut self) -> bool {
        if !self.paused() {
            return false;
        }
        self.state = TimeRef::Absolute {
            starts_at: self.starts_at(),
        };
        self.player_state_id = Uuid::new_v4();
        true
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

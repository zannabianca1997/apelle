use std::collections::HashMap;

use apelle_configs_dtos::QueueConfig;
use apelle_songs_dtos::public::Song;
use chrono::{DateTime, Duration, FixedOffset, Local};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Queue {
    pub id: Uuid,
    pub code: String,

    pub current: Option<Current>,

    pub config: QueueConfig,

    pub queue: HashMap<Uuid, QueuedSong>,

    pub created: DateTime<FixedOffset>,
    pub updated: DateTime<FixedOffset>,
}

#[derive(Debug, Clone)]
pub struct Current {
    pub song: Song,
    pub player_state_id: Uuid,

    state: TimeRef,
}

#[derive(Debug, Clone, Copy)]
pub enum TimeRef {
    Relative { position: Duration },
    Absolute { starts_at: DateTime<FixedOffset> },
}

impl Current {
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
    pub fn stopped(&self) -> bool {
        match self.state {
            TimeRef::Relative { .. } => true,
            TimeRef::Absolute { starts_at } => {
                Local::now().signed_duration_since(starts_at) >= self.song.duration
            }
        }
    }

    /// If the song is playing
    pub fn playing(&self) -> bool {
        !self.stopped()
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
        true
    }
}

#[derive(Debug, Clone)]
pub struct QueuedSong {
    pub song: Song,
    pub queued_at: DateTime<FixedOffset>,
    pub likes: u16,
}

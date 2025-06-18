use std::collections::HashMap;

use apelle_configs_dtos::QueueConfig;
use apelle_songs_dtos::public::Song;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
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
    song: Song,
    player_state_id: Uuid,
    state: TimeRef,
}

#[derive(Debug, Clone, Copy)]
enum TimeRef {
    Relative { position: Duration },
    Absolute { starts_at: DateTime<FixedOffset> },
}

impl Current {
    pub fn new(song: Song) -> Self {
        Self {
            song,
            player_state_id: Uuid::new_v4(),
            state: TimeRef::Relative {
                position: Duration::zero(),
            },
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

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CurrentDto<'a> {
    pub song: &'a Song,
    pub player_state_id: Uuid,

    /// Position in the song
    #[serde(with = "apelle_common::iso8601::duration")]
    pub position: Duration,
    /// Point in time when the song should have started to reach the current
    pub starts_at: DateTime<FixedOffset>,
    /// If the song is stopped
    pub stopped: bool,
    /// If the song was manually paused
    pub paused: bool,
}

impl Serialize for Current {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        CurrentDto::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Current> for CurrentDto<'a> {
    fn from(value: &'a Current) -> Self {
        Self {
            song: &value.song,
            player_state_id: value.player_state_id,
            position: value.position(),
            starts_at: value.starts_at(),
            stopped: value.stopped(),
            paused: value.paused(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct QueuedSong {
    pub song: Song,
    pub queued_at: DateTime<FixedOffset>,
    pub likes: u16,
}

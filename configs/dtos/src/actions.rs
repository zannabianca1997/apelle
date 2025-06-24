use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{
    Decode, Encode, Postgres, Type,
    postgres::{PgHasArrayType, PgTypeInfo},
};
use strum::{EnumIter, IntoEnumIterator};
use utoipa::{
    PartialSchema, ToSchema,
    openapi::{self, Object},
};

macro_rules! conversions {
(
    $typ:ident: $( $var:ident <=> $value:literal ),*
) => {
    impl $typ {
        pub const fn as_str(&self) -> &'static str {
            match self {
                $(
                    $typ::$var => $value,
                )*
            }
        }
    }

    impl From<$typ> for &'static str {
        fn from(value: $typ) -> Self {
            value.as_str()
        }
    }

    impl<'a> TryFrom<&'a str> for $typ {
        type Error = &'a str;

        fn try_from(value: &'a str) -> Result<Self, Self::Error> {
            match value {
                $(
                    $value => Ok($typ::$var),
                )*
                _ => Err(value),
            }
        }
    }
}
}

macro_rules! other_impls {
    ($typ:ident) => {
        impl Display for $typ {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <&str>::from(*self).fmt(f)
            }
        }

        impl FromStr for $typ {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::try_from(s).map_err(ToOwned::to_owned)
            }
        }

        impl<'a, DB> Encode<'a, DB> for $typ
        where
            DB: sqlx::Database,
            &'static str: Encode<'a, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'a>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                <&'static str as Encode<DB>>::encode((*self).into(), buf)
            }

            fn encode(
                self,
                buf: &mut <DB as sqlx::Database>::ArgumentBuffer<'a>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError>
            where
                Self: Sized,
            {
                <&'static str as Encode<DB>>::encode_by_ref(&self.into(), buf)
            }

            fn produces(&self) -> Option<<DB as sqlx::Database>::TypeInfo> {
                <&'static str as Encode<DB>>::produces(&(*self).into())
            }

            fn size_hint(&self) -> usize {
                <&'static str as Encode<DB>>::size_hint(&(*self).into())
            }
        }

        impl<'a, 'r, DB> Decode<'r, DB> for $typ
        where
            DB: sqlx::Database,
            &'a str: Decode<'r, DB>,
        {
            fn decode(
                value: <DB as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                <&str as Decode<DB>>::decode(value)
                    .and_then(|s| s.try_into().map_err(sqlx::error::BoxDynError::from))
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "&str")]
pub enum QueueUserAction {
    Queue(QueueUserActionQueue),
    Song(QueueUserActionSong),
    User(QueueUserActionUser),
}

impl From<QueueUserAction> for &'static str {
    fn from(value: QueueUserAction) -> Self {
        match value {
            QueueUserAction::Queue(action) => action.into(),
            QueueUserAction::Song(action) => action.into(),
            QueueUserAction::User(action) => action.into(),
        }
    }
}

impl PartialSchema for QueueUserAction {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        openapi::RefOr::T(
            Object::builder()
                .description(Some("An action a user makes on a queue"))
                .schema_type(openapi::Type::String)
                .enum_values(Some(QueueUserAction::iter().map(<&str>::from)))
                .build()
                .into(),
        )
    }
}
impl ToSchema for QueueUserAction {}

impl<'a> TryFrom<&'a str> for QueueUserAction {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Err(value)
            .or_else(|value| QueueUserActionQueue::try_from(value).map(QueueUserAction::Queue))
            .or_else(|value| QueueUserActionSong::try_from(value).map(QueueUserAction::Song))
            .or_else(|value| QueueUserActionUser::try_from(value).map(QueueUserAction::User))
    }
}

impl Type<Postgres> for QueueUserAction {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        PgTypeInfo::with_name("queue_user_action")
    }
}

impl PgHasArrayType for QueueUserAction {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::array_of("queue_user_action")
    }
}

impl QueueUserAction {
    pub fn iter() -> impl Iterator<Item = Self> {
        QueueUserActionQueue::iter()
            .map(QueueUserAction::Queue)
            .chain(QueueUserActionSong::iter().map(QueueUserAction::Song))
            .chain(QueueUserActionUser::iter().map(QueueUserAction::User))
    }
}

other_impls! {QueueUserAction}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(try_from = "&str", into = "&str")]
pub enum QueueUserActionQueue {
    /// Get the queue data
    Get,
    /// Delete the queue
    Delete,
    /// Change the queue configuration
    Configure,
}

conversions! {
    QueueUserActionQueue:
        Get <=> "GET_QUEUE",
        Delete <=> "DELETE_QUEUE",
        Configure <=> "CONFIGURE_QUEUE"
}
other_impls! {QueueUserActionQueue}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(try_from = "&str", into = "&str")]
pub enum QueueUserActionSong {
    /// Remove from queue
    Remove,

    /// Ban from queue
    Ban,
    /// Unban from queue
    Unban,

    /// Add to queue
    Enqueue,

    // Start playing
    Play,
    /// Pause the playing song
    Pause,

    /// Change the song with another one (usually the next in list)
    Next,
    /// Softer version of [`QueueUserActionSong::Next`]
    ///
    /// Can be called only on a song that is finished without user interaction,
    /// and only to change to the next song in the queue
    AutoNext,

    /// Like the song
    Like,
}

conversions! {
    QueueUserActionSong:
        Remove <=> "REMOVE_SONG",
        Ban <=> "BAN_SONG",
        Unban <=> "UNBAN_SONG",
        Enqueue <=> "ENQUEUE_SONG",
        Play <=> "PLAY_SONG",
        Pause <=> "PAUSE_SONG",
        Next <=> "NEXT_SONG",
        AutoNext <=> "AUTO_NEXT_SONG",
        Like <=> "LIKE_SONG"
}
other_impls! {QueueUserActionSong}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(try_from = "&str", into = "&str")]
pub enum QueueUserActionUser {
    /// Ban the user
    Ban,
    /// Unban the user
    Unban,
    /// Remove the user from the queue
    ///
    /// This also removes all songs added by the user, and all likes given by
    /// the user
    Remove,
}

conversions! {
    QueueUserActionUser:
        Ban <=> "BAN_USER",
        Unban <=> "UNBAN_USER",
        Remove <=> "REMOVE_USER"
}
other_impls! {QueueUserActionUser}

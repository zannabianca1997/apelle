use derive_more::{IsVariant, TryUnwrap, Unwrap};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Either an id or a representation of a resource
#[derive(Debug, Clone, Serialize, Deserialize, TryUnwrap, Unwrap, IsVariant, ToSchema)]
#[serde(untagged)]
pub enum IdOrRep<T> {
    Id(Uuid),
    Rep(T),
}

impl<T> IdOrRep<T> {
    pub async fn or_extract(self, extract: impl AsyncFnOnce(Uuid) -> T) -> T {
        match self {
            IdOrRep::Id(id) => extract(id).await,
            IdOrRep::Rep(rep) => rep,
        }
    }

    pub async fn or_try_extract<E>(
        self,
        extract: impl AsyncFnOnce(Uuid) -> Result<T, E>,
    ) -> Result<T, E> {
        match self {
            IdOrRep::Id(id) => extract(id).await,
            IdOrRep::Rep(rep) => Ok(rep),
        }
    }

    pub async fn or_extract_inplace(&mut self, extract: impl AsyncFnOnce(Uuid) -> T) -> &mut T {
        match self {
            IdOrRep::Id(id) => {
                *self = IdOrRep::Rep(extract(*id).await);
                let IdOrRep::Rep(t) = self else {
                    unreachable!()
                };
                t
            }
            IdOrRep::Rep(t) => t,
        }
    }

    pub async fn or_try_extract_inplace<E>(
        &mut self,
        extract: impl AsyncFnOnce(Uuid) -> Result<T, E>,
    ) -> Result<&mut T, E> {
        match self {
            IdOrRep::Id(id) => {
                *self = IdOrRep::Rep(extract(*id).await?);
                let IdOrRep::Rep(t) = self else {
                    unreachable!()
                };
                Ok(t)
            }
            IdOrRep::Rep(t) => Ok(t),
        }
    }
}

impl<T> From<Uuid> for IdOrRep<T> {
    fn from(id: Uuid) -> Self {
        Self::Id(id)
    }
}

pub trait HasId {
    fn id(&self) -> Uuid;
}

impl<T: HasId> HasId for IdOrRep<T> {
    fn id(&self) -> Uuid {
        match self {
            IdOrRep::Id(id) => *id,
            IdOrRep::Rep(t) => t.id(),
        }
    }
}

impl HasId for Uuid {
    fn id(&self) -> Uuid {
        *self
    }
}

impl<T: HasId> HasId for &T {
    fn id(&self) -> Uuid {
        T::id(self)
    }
}

impl<T: HasId> HasId for &mut T {
    fn id(&self) -> Uuid {
        T::id(self)
    }
}

impl<T: HasId> HasId for Box<T> {
    fn id(&self) -> Uuid {
        T::id(self)
    }
}

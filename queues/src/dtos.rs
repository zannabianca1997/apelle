use apelle_configs_dtos::QueueConfigCreate;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct QueueCreate {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub config: Config,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(untagged)]
pub enum Config {
    Existing(Uuid),
    New(QueueConfigCreate),
}

impl Default for Config {
    fn default() -> Self {
        Self::Existing(Uuid::nil())
    }
}

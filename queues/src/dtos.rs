use apelle_configs_dtos::QueueConfigCreate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone, Debug)]
pub struct QueueCreate {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub config: Config,
}

#[derive(Deserialize, Clone, Debug)]
pub enum Config {
    Existing(Uuid),
    New(QueueConfigCreate),
}

impl Default for Config {
    fn default() -> Self {
        Self::Existing(Uuid::nil())
    }
}

use apelle_common::{
    Figment, ProvideDefaults, Provider, Serialized, db::migrations::MigrationsConfigs,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,

    pub migrate: MigrationsConfigs,
}
impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new().join(Serialized::default("migrate", MigrationsConfigs::default()))
    }
}

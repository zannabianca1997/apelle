use apelle_common::{
    Figment, ProvideDefaults, Provider, Serialized, db::migrations::MigrationsConfigs,
};
use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Database connection string
    pub db_url: Url,

    /// Size of the queue for the login date updater
    pub login_queue_size: usize,

    pub migrate: MigrationsConfigs,
}

impl ProvideDefaults for Config {
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Figment::new()
            .join(("login_queue_size", 10))
            .join(Serialized::default("migrate", MigrationsConfigs::default()))
    }
}

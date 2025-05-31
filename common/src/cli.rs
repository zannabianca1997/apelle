use std::{collections::BTreeMap, path::PathBuf};

use figment::{
    Figment, Metadata, Profile, Provider,
    providers::{Env, Format as _, Serialized, Toml},
    value::{Dict, Map, Tag},
};
use serde::{
    Serialize,
    de::{DeserializeOwned, Error as _},
};

use crate::main_wrapper::CommonConfig;

#[derive(Debug, Clone, clap::Parser)]
pub struct CliArgs {
    /// Configuration file. If not specified, will look for `Apelle.toml` in the current directory
    /// or in one of the ancestors
    #[clap(short, long)]
    pub config_file: Option<PathBuf>,
    /// Do not search for the default config file
    #[clap(long)]
    pub no_default_config_file: bool,
    /// Do not load enviroment variables
    #[clap(long)]
    no_env: bool,
    /// Do not use defaults, load every config from other sources
    #[clap(long)]
    no_defaults: bool,
    /// Profile to use from the configuration. Defaults to the name of the service
    #[clap(long)]
    profile: Option<String>,
    /// Command line configuration
    ///
    /// Formatted as `conf.name=value`. Will overwrite any other source.
    #[clap(short = 'C')]
    configs: Vec<String>,
}
impl Provider for CliArgs {
    fn metadata(&self) -> Metadata {
        Metadata::named("command line arguments")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        let mut data = Dict::new();

        for item in &self.configs {
            let (n, v) = item.split_once('=').ok_or_else(|| {
                figment::Error::custom(format!(
                    "Invalid configuration {item}: expected string of the type `conf.name=value`"
                ))
            })?;
            let mut data = &mut data;
            let mut components = n.split('.');
            let name = components.next_back().unwrap();
            for component in components {
                let figment::value::Value::Dict(_, new_data) =
                    data.entry(component.to_string()).or_insert_with(|| {
                        figment::value::Value::Dict(Tag::Default, BTreeMap::default())
                    })
                else {
                    unreachable!()
                };
                data = new_data;
            }

            data.insert(name.to_owned(), v.parse().unwrap());
        }

        Ok(Map::from_iter([(Profile::Global, data)]))
    }
}

impl CliArgs {
    pub fn get_configuration<AppConfig>(
        self,
        service_name: &str,
        service_default_port: u16,
    ) -> figment::Result<(AppConfig, CommonConfig)>
    where
        AppConfig: ProvideDefaults + DeserializeOwned,
    {
        // First, the defaults values
        let mut figment = if self.no_defaults {
            Figment::new()
        } else {
            Figment::from(CommonConfig::defaults(service_name, service_default_port))
                .merge(AppConfig::defaults(service_name, service_default_port))
        }
        .select(self.profile.as_deref().unwrap_or(service_name));
        // Then the default config file
        if !self.no_default_config_file {
            figment = figment
                .merge(Toml::file("Apelle.toml").nested())
                .merge(Toml::file(format!("Apelle-{service_name}.toml")).profile(service_name));
        }
        // Then the one provided by the user
        if let Some(config_file) = &self.config_file {
            figment = figment.merge(Toml::file_exact(config_file).profile(Profile::Global));
        }
        // Then, the enviroment variables and the arguments
        if !self.no_env {
            match dotenvy::dotenv() {
                Ok(_) => (),
                Err(err) if err.not_found() => (),
                Err(err) => eprintln!("Cannot open `.env` to load enviroment variable: {err}"),
            };
            figment = figment.merge(Env::prefixed("APELLE__").split("__").global());
        }
        // Finally the cli arguments
        let figment = figment.merge(self);

        let app = figment.extract()?;
        let common = figment.extract()?;

        Ok((app, common))
    }
}

pub trait ProvideDefaults {
    fn defaults(service_name: &str, service_default_port: u16) -> impl Provider;
}

impl<T> ProvideDefaults for T
where
    T: Default + Serialize,
{
    fn defaults(_service_name: &str, _service_default_port: u16) -> impl Provider {
        Serialized::defaults(T::default())
    }
}

use crate::configs::consts::{get_default_config_path, DEFAULT_CONFIG, ENV_DEVELOPMENT};
use crate::configs::database::Database;
use anyhow::{Context, Result};
use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Conf {
    pub host: String,
    pub port: i32,
    pub database: Database,
    pub auth_token: String,
}

impl Conf {
    pub fn new() -> Result<Self> {
        // Get config from env
        let mut c = Config::default();

        // Get default config
        let default_conf = get_default_config_path(DEFAULT_CONFIG);
        c.merge(File::with_name(default_conf.as_str()))
            .context("Unable to load the default config")?;

        // Get env config
        let e = env::var("ENV").unwrap_or_else(|_| ENV_DEVELOPMENT.into());
        let env_conf = get_default_config_path(e.as_str());
        c.merge(File::with_name(env_conf.as_str()).required(false))
            .context(format!("Unable to load config/{}", e))?;

        // Add env to config
        c.merge(Environment::new().separator("_"))?;

        // Build config
        c.try_into().context("Unable to instantiate Config struct")
    }
}

// Init Config
lazy_static! {
    pub static ref CONFIG: Conf = Conf::new().unwrap();
}

impl Debug for CONFIG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Conf")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("auth_token", &self.auth_token)
            .finish()
    }
}

use serde::Deserialize;
use config::{Config, ConfigError, Environment};

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            // We DON'T load the file here anymore. 
            // We tell it to look at the system environment variables.
            // use double-underscore separator to match .env keys like `SERVER.HOST` and `PG.HOST`
            .add_source(Environment::default().separator(".").try_parsing(true))
            .build()?;

        cfg.try_deserialize()
    }
}
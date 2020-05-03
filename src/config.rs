//! Configuration functions for use with the [`rocket::Rocket`] instance.

use anyhow::Result;
use rocket::config::{Config, Environment};
use serde::Deserialize;

/// Configuration values that are read from environment variables.
#[derive(Deserialize)]
pub struct EnvConfig {
    /// The TCP port to listen on. Defaults to `8080` if not set.
    port: Option<u16>,
    /// Amount of workers to run for the server. Defaults to `4` if not set.
    workers: Option<u16>,
    /// Secret key used for private cookies (optional in debug mode). If missing, a random key is
    /// generated on each start up.
    #[cfg(debug_assertions)]
    secret_key: Option<String>,
    /// Secret key used for private cookies.
    #[cfg(not(debug_assertions))]
    secret_key: String,
}

/// Load a Rocket [`Config`] based on custom environment variables.
pub fn load() -> Result<Config> {
    let env_config = envy::from_env::<EnvConfig>()?;

    let environment = if cfg!(debug_assertions) {
        Environment::Development
    } else {
        Environment::Production
    };

    let config = Config::build(environment)
        .port(env_config.port.unwrap_or(8080))
        .workers(env_config.workers.unwrap_or(4));

    #[cfg(debug_assertions)]
    let config = if let Some(secret_key) = env_config.secret_key {
        config.secret_key(secret_key)
    } else {
        config
    };

    #[cfg(not(debug_assertions))]
    let config = config.secret_key(env_config.secret_key);

    Ok(config.finalize()?)
}

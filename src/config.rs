//! Configuration functions for use with the [`rocket::Rocket`] instance.

use anyhow::Result;
use rocket::config::{Config as RocketConfig, Environment};
use serde::Deserialize;

/// Configuration values that are read from a configuration file.
#[derive(Deserialize)]
pub struct Config {
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
    /// Public facing URL from where the service is accessible.
    pub host: String,
    /// Settings for an email SMTP client.
    pub smtp: SmtpConfig,
}

/// Configuration values to configure a SMTP client for sending emails.
#[derive(Deserialize)]
pub struct SmtpConfig {
    /// Domain name of the server.
    pub domain: String,
    /// Port to connect to.
    pub port: u16,
    /// Username for authentication (usually the email address).
    pub username: String,
    /// Password for authentication.
    pub password: String,
}

/// Load a Rocket [`RocketConfig`] based on custom configuration file.
pub fn load() -> Result<(RocketConfig, Config)> {
    let file_config = load_file()?;

    let environment = if cfg!(debug_assertions) {
        Environment::Development
    } else {
        Environment::Production
    };

    let config = RocketConfig::build(environment)
        .port(file_config.port.unwrap_or(8080))
        .workers(file_config.workers.unwrap_or(4));

    #[cfg(debug_assertions)]
    let config = if let Some(ref secret_key) = file_config.secret_key {
        config.secret_key(secret_key)
    } else {
        config
    };

    #[cfg(not(debug_assertions))]
    let config = config.secret_key(&file_config.secret_key);

    Ok((config.finalize()?, file_config))
}

/// Load the configuration from a fixed file path. The location can be overridden with the
/// `CONFIG_FILE` environment variable.
#[cfg(not(test))]
fn load_file() -> Result<Config> {
    use std::{env, fs};

    let path = env::var("CONFIG_FILE").unwrap_or_else(|_| String::from("/app/amelio.toml"));
    let file = fs::read(path)?;

    toml::de::from_slice::<Config>(&file).map_err(Into::into)
}

#[cfg(test)]
fn load_file() -> Result<Config> {
    Ok(Config {
        port: None,
        workers: None,
        secret_key: None,
        host: "http://localhost:8080".to_owned(),
        smtp: SmtpConfig {
            domain: String::new(),
            port: 0,
            username: String::new(),
            password: String::new(),
        },
    })
}

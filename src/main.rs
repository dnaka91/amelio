#![feature(proc_macro_hygiene, decl_macro)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

use anyhow::Result;
use log::info;
use rocket::{routes, Rocket};

mod config;
mod routes;
mod templates;

/// Create a new pre-configured [`Rocket`] instance.
fn rocket() -> Result<Rocket> {
    let config = config::load()?;

    Ok(rocket::custom(config).mount("/", routes![routes::index]))
}

fn main() {
    dotenv::dotenv().ok();

    ctrlc::set_handler(|| {
        info!("Shutting down");
        std::process::exit(1);
    })
    .expect("Error setting shutdown handler");

    rocket().unwrap().launch();
}

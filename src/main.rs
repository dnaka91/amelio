#![feature(proc_macro_hygiene, decl_macro)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use anyhow::Result;
use log::info;
use rocket::{routes, Rocket};

use crate::db::connection::DbConn;
use crate::db::DbMigrations;

mod config;
mod db;
mod roles;
mod routes;
mod services;
mod templates;

/// Create a new pre-configured [`Rocket`] instance.
fn rocket() -> Result<Rocket> {
    let config = config::load()?;

    Ok(rocket::custom(config)
        .attach(DbConn::fairing())
        .attach(DbMigrations::fairing())
        .mount(
            "/",
            routes![
                routes::index_user,
                routes::index,
                routes::auth::login,
                routes::auth::post_login,
                routes::auth::post_logout
            ],
        ))
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

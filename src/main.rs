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
use rocket::{catchers, routes, Rocket};

use crate::db::connection::DbConn;
use crate::db::DbMigrations;

mod config;
mod db;
mod email;
mod hashing;
mod models;
mod roles;
mod routes;
mod services;
mod templates;

/// Create a new pre-configured [`Rocket`] instance.
fn rocket() -> Result<Rocket> {
    let (rocket_config, config) = config::load()?;

    Ok(rocket::custom(rocket_config)
        .attach(DbConn::fairing())
        .attach(DbMigrations::fairing())
        .manage(config)
        .mount(
            "/",
            routes![
                routes::index_user,
                routes::index,
                routes::auth::login,
                routes::auth::post_login,
                routes::auth::post_logout,
                // Assets should always be last
                routes::assets::assets,
            ],
        )
        .mount(
            "/users",
            routes![
                routes::users::users_admin,
                routes::users::users_auth,
                routes::users::users,
                routes::users::new_user_admin,
                routes::users::new_user_auth,
                routes::users::new_user,
                routes::users::post_new_user_admin,
                routes::users::post_new_user_auth,
                routes::users::post_new_user,
                routes::users::activate,
                routes::users::post_activate,
            ],
        )
        .register(catchers![
            routes::errors::forbidden,
            routes::errors::not_found
        ]))
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

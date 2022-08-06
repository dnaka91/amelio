//! # Amelio - A university project for the IUBH
//!
//! Amelio is a group project for the IUBH in Germany. It is a ticket system that helps to report
//! and track errors in study media.
//!
//! The name Amelio is a short version of _[ameliorate]_ and is another word for **improve**.
//!
//! [ameliorate]: https://www.dictionary.com/browse/ameliorate

#![feature(proc_macro_hygiene, decl_macro)]
#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(
    unused_extern_crates,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value
)]

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
mod dirs;
mod email;
mod fairings;
mod hashing;
mod language;
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
        .attach(fairings::Csp)
        .attach(fairings::Auth)
        .manage(config)
        .mount(
            "/",
            routes![
                routes::fairing::forbidden,
                routes::fairing::to_login,
                routes::index_user,
                routes::index,
                routes::faq_user,
                routes::faq,
                routes::auth::login,
                routes::auth::post_login,
                routes::auth::post_logout,
                routes::users::activate,
                routes::users::post_activate,
                // Assets should always be last
                routes::assets::get,
            ],
        )
        .mount(
            "/users",
            routes![
                routes::users::list,
                routes::users::new,
                routes::users::post_new,
                routes::users::enable,
                routes::users::edit,
                routes::users::post_edit,
            ],
        )
        .mount(
            "/courses",
            routes![
                routes::courses::list,
                routes::courses::new,
                routes::courses::post_new,
                routes::courses::enable,
                routes::courses::edit,
                routes::courses::post_edit,
            ],
        )
        .mount(
            "/tickets",
            routes![
                routes::tickets::new,
                routes::tickets::post_new,
                routes::tickets::edit,
                routes::tickets::post_edit,
                routes::tickets::post_add_comment,
                routes::tickets::forward,
                routes::tickets::change_status,
                routes::tickets::search,
            ],
        )
        .register(catchers![
            routes::errors::forbidden,
            routes::errors::not_found,
            routes::errors::internal_server_error,
        ]))
}

fn main() {
    ctrlc::set_handler(|| {
        info!("Shutting down");
        std::process::exit(1);
    })
    .expect("Error setting shutdown handler");

    rocket().unwrap().launch();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::{ContentType, Status};
    use rocket::local::{Client, LocalResponse};
    use rocket::uri;

    use crate::routes;

    pub fn prepare_logged_in_client(username: &str, password: &str) -> Client {
        let client = Client::new(crate::rocket().unwrap()).unwrap();

        {
            let res = client
                .post(uri!(routes::auth::login).to_string())
                .body(format!("username={}&password={}", username, password))
                .header(ContentType::Form)
                .dispatch();

            assert_eq!(Status::SeeOther, res.status());
            assert_eq!(Some("/"), res.headers().get_one("Location"));
        }

        client
    }

    pub fn check_form<'a, B: AsRef<str>>(
        client: &'a Client,
        uri: &'a str,
        body: B,
    ) -> LocalResponse<'a> {
        client
            .post(uri)
            .body(body.as_ref())
            .header(ContentType::Form)
            .dispatch()
    }
}

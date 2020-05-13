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
mod fairings;
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
                routes::auth::login,
                routes::auth::post_login,
                routes::auth::post_logout,
                routes::users::activate,
                routes::users::post_activate,
                // Assets should always be last
                routes::assets::assets,
            ],
        )
        .mount(
            "/users",
            routes![
                routes::users::users,
                routes::users::new_user,
                routes::users::post_new_user,
                routes::users::enable_user,
            ],
        )
        .mount(
            "/courses",
            routes![
                routes::courses::courses,
                routes::courses::new_course,
                routes::courses::post_new_course,
                routes::courses::enable_course,
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

    pub fn check_form<'a>(client: &'a Client, uri: &'a str, body: &'a str) -> LocalResponse<'a> {
        client
            .post(uri)
            .body(body)
            .header(ContentType::Form)
            .dispatch()
    }
}

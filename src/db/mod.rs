//! All database related functionality.

#![allow(clippy::wildcard_imports)]

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::fairing::{AdHoc, Fairing};

use self::connection::DbConn;

pub mod connection;
pub mod models;
pub mod repositories;
pub mod schema;

embed_migrations!("migrations");

/// Database migrations that will be run when [`attach`](rocket::Rocket::attach)ed to a Rocket
/// instance.
pub struct DbMigrations;

impl DbMigrations {
    /// Create a fairing for Rocket.
    pub fn fairing() -> impl Fairing {
        AdHoc::on_attach("Database Migrations", |rocket| {
            if let Some(conn) = DbConn::get_one(&rocket) {
                if let Err(e) = init(&*conn) {
                    rocket::logger::error(&format!("Database initialization failed: {:?}", e));
                    Err(rocket)
                } else {
                    Ok(rocket)
                }
            } else {
                rocket::logger::error("No database connection");
                Err(rocket)
            }
        })
    }
}

/// Initialize the database by running any outstanding migrations, creating the initial user if none
/// exist.
fn init(conn: &SqliteConnection) -> Result<()> {
    embedded_migrations::run(conn).context("database migrations failed")?;
    create_admin_user(conn).context("admin user creation failed")?;
    Ok(())
}

/// Create the initial admin user.
fn create_admin_user(conn: &diesel::SqliteConnection) -> Result<()> {
    use crate::db::models::NewUser;
    use crate::db::schema::users::dsl::*;

    if users.count().get_result::<i64>(conn)? > 0 {
        return Ok(());
    }

    diesel::insert_into(users)
        .values(&NewUser {
            username: "admin",
            password: "admin",
            name: "Administrator",
            role: "admin",
        })
        .execute(&*conn)?;

    Ok(())
}

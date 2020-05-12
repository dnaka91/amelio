//! All database related functionality.

#![allow(clippy::wildcard_imports)]

use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::fairing::{AdHoc, Fairing};

use self::connection::DbConn;
use self::models::{InitCourseEntity, InitUserEntity};

use crate::hashing::{self, Hasher};

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
    create_sample_users(conn).context("sample users creation failed")?;
    create_sample_courses(conn).context("sample courses creation failed")?;
    Ok(())
}

/// Create the initial admin user.
fn create_admin_user(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::users::dsl::*;

    if users.count().get_result::<i64>(conn)? > 0 {
        return Ok(());
    }

    let hasher = hashing::new_hasher();

    diesel::insert_into(users)
        .values(&InitUserEntity {
            username: "admin",
            password: &hasher.hash("admin")?,
            name: "Administrator",
            role: "admin",
            active: true,
        })
        .execute(&*conn)?;

    Ok(())
}

/// Create several sample users for testing purposes.
fn create_sample_users(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::users::dsl::*;

    if users.count().get_result::<i64>(conn)? >= 7 {
        return Ok(());
    }

    let hasher = hashing::new_hasher();

    diesel::insert_into(users)
        .values(vec![
            &InitUserEntity {
                username: "student1",
                password: &hasher.hash("student1")?,
                name: "Max Mustermann",
                role: "student",
                active: true,
            },
            &InitUserEntity {
                username: "student2",
                password: &hasher.hash("student2")?,
                name: "Maria Meister",
                role: "student",
                active: true,
            },
            &InitUserEntity {
                username: "sleeper1",
                password: &hasher.hash("sleeper1")?,
                name: "Bernd Faultier",
                role: "student",
                active: false,
            },
            &InitUserEntity {
                username: "sleeper2",
                password: &hasher.hash("sleeper2")?,
                name: "Regina Schlafmaus",
                role: "student",
                active: false,
            },
            &InitUserEntity {
                username: "autor1",
                password: &hasher.hash("autor1")?,
                name: "Siegfried Siegreich",
                role: "author",
                active: true,
            },
            &InitUserEntity {
                username: "tutor1",
                password: &hasher.hash("tutor1")?,
                name: "Frieda Freundlich",
                role: "tutor",
                active: true,
            },
        ])
        .execute(&*conn)?;

    Ok(())
}

/// Create several sample courses for testing purposes.
fn create_sample_courses(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::courses::dsl::*;

    if courses.count().get_result::<i64>(conn)? >= 3 {
        return Ok(());
    }

    diesel::insert_into(courses)
        .values(vec![
            &InitCourseEntity {
                code: "TEST01",
                title: "Testkurs 1",
                author_id: 1,
                tutor_id: 1,
                active: true,
            },
            &InitCourseEntity {
                code: "TEST02",
                title: "Testkurs 2",
                author_id: 6,
                tutor_id: 7,
                active: true,
            },
            &InitCourseEntity {
                code: "TEST03",
                title: "Testkurs 3",
                author_id: 6,
                tutor_id: 7,
                active: false,
            },
        ])
        .execute(&*conn)?;

    Ok(())
}

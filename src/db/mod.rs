//! All database related functionality.

#![allow(clippy::wildcard_imports)]

use std::convert::TryFrom;
use std::iter;

use anyhow::{Context, Result};
use chrono::NaiveTime;
use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::query_builder::QueryFragment;
use diesel::result::Error as DieselError;
use diesel::sqlite::Sqlite;
use diesel::SqliteConnection;

use rand::distributions::Alphanumeric;
use rand::Rng;
use rocket::fairing::{AdHoc, Fairing};
use serde::Deserialize;
use url::Url;

use self::connection::DbConn;
use self::models::{
    InitCourseEntity, InitTicketEntity, InitUserEntity, MediumInteractiveEntity,
    MediumQuestionaireEntity, MediumRecordingEntity, MediumTextEntity,
};
use crate::hashing::{self, Hasher};
use crate::models::{Category, Priority, Role, Status, TicketType};

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

#[derive(strum::EnumString, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
enum Samples {
    Users,
    Courses,
    Tickets,
}

/// Initialize the database by running any outstanding migrations, creating the initial user if none
/// exist.
fn init(conn: &SqliteConnection) -> Result<()> {
    embedded_migrations::run(conn).context("database migrations failed")?;
    create_admin_user(conn).context("admin user creation failed")?;
    create_sample_users(conn).context("sample users creation failed")?;
    create_sample_courses(conn).context("sample courses creation failed")?;
    create_sample_tickets(conn).context("sample tickets creation failed")?;
    Ok(())
}

fn created(conn: &SqliteConnection, sample: Samples) -> Result<bool> {
    use crate::db::schema::samples::dsl::*;

    let res = samples
        .find(sample.as_ref())
        .select(created)
        .get_result::<bool>(conn);

    if res == Err(DieselError::NotFound) {
        return Ok(false);
    }

    res.map_err(Into::into)
}

fn set_created(conn: &SqliteConnection, sample: Samples) -> Result<()> {
    use crate::db::schema::samples::dsl::*;

    let res = diesel::update(samples.find(sample.as_ref()))
        .set(created.eq(true))
        .execute(conn)?;

    if res == 0 {
        diesel::insert_into(samples)
            .values((id.eq(sample.as_ref()), created.eq(true)))
            .execute(conn)?;
    }

    Ok(())
}

/// Create the initial admin user.
fn create_admin_user(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::users;

    if users::table.count().get_result::<i64>(conn)? >= 1 {
        return Ok(());
    }

    let hasher = hashing::new_hasher();
    let mut rng = rand::thread_rng();

    let password = if cfg!(test) {
        "admin".to_owned()
    } else {
        let password = iter::repeat(())
            .map(|_| rng.sample(Alphanumeric))
            .map(char::from)
            .take(16)
            .collect::<String>();

        log::warn!("Initial admin password is: {}", password);
        password
    };

    diesel::insert_into(users::table)
        .values(&InitUserEntity {
            username: "admin".to_owned(),
            password: hasher.hash(&password)?,
            name: "Administrator".to_owned(),
            role: "admin".to_owned(),
            active: true,
        })
        .execute(&*conn)?;

    Ok(())
}

const USERS_JSON: &[u8] = include_bytes!("import/users.json");

/// Create several sample users for testing purposes.
fn create_sample_users(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::users::dsl::*;

    if created(conn, Samples::Users)? {
        return Ok(());
    }

    let mut values = serde_json::from_slice::<Vec<InitUserEntity>>(USERS_JSON)?;
    let hasher = hashing::new_hasher();

    for user in &mut values {
        user.role.parse::<Role>()?;
        user.password = hasher.hash(&user.password)?;
    }

    diesel::insert_into(users).values(values).execute(&*conn)?;

    set_created(conn, Samples::Users)?;
    Ok(())
}

const COURSES_JSON: &[u8] = include_bytes!("import/courses.json");

/// Create several sample courses for testing purposes.
fn create_sample_courses(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::courses::dsl::*;

    if created(conn, Samples::Courses)? {
        return Ok(());
    }

    let values = serde_json::from_slice::<Vec<InitCourseEntity>>(COURSES_JSON)?;

    diesel::insert_into(courses)
        .values(values)
        .execute(&*conn)?;

    set_created(conn, Samples::Courses)?;
    Ok(())
}

#[derive(Deserialize)]
struct TicketData {
    tickets: Vec<InitTicketEntity>,
    texts: Vec<MediumTextEntity>,
    recordings: Vec<MediumRecordingEntity>,
    interactives: Vec<MediumInteractiveEntity>,
    questionaires: Vec<MediumQuestionaireEntity>,
}

const TICKETS_JSON: &[u8] = include_bytes!("import/tickets.json");

/// Create several sample tickets for testing purposes.
fn create_sample_tickets(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::{
        medium_interactives, medium_questionaires, medium_recordings, medium_texts, tickets,
    };

    if created(conn, Samples::Tickets)? {
        return Ok(());
    }

    let values = serde_json::from_slice::<TicketData>(TICKETS_JSON)?;

    for t in &values.tickets {
        t.type_.parse::<TicketType>()?;
        t.category.parse::<Category>()?;
        t.priority.parse::<Priority>()?;
        t.status.parse::<Status>()?;
    }

    for t in &values.texts {
        u16::try_from(t.page)?;
        u16::try_from(t.line)?;
    }

    for r in &values.recordings {
        NaiveTime::parse_from_str(&r.time, "%H:%M:%S")?;
    }

    for i in &values.interactives {
        Url::parse(&i.url)?;
    }

    for q in &values.questionaires {
        u16::try_from(q.question)?;
    }

    diesel::insert_into(tickets::table)
        .values(values.tickets)
        .execute(&*conn)?;

    diesel::insert_into(medium_texts::table)
        .values(values.texts)
        .execute(&*conn)?;

    diesel::insert_into(medium_recordings::table)
        .values(values.recordings)
        .execute(&*conn)?;

    diesel::insert_into(medium_interactives::table)
        .values(values.interactives)
        .execute(&*conn)?;

    diesel::insert_into(medium_questionaires::table)
        .values(values.questionaires)
        .execute(&*conn)?;

    set_created(conn, Samples::Tickets)?;
    Ok(())
}

/// Extensions for query fragments in [`diesel`] queries.
trait QueryExt<DB, T>
where
    DB: Backend,
    DB::QueryBuilder: Default,
    T: QueryFragment<DB>,
{
    /// Log the current query.
    fn log_query(self) -> Self;
}

impl<T> QueryExt<Sqlite, T> for T
where
    T: QueryFragment<Sqlite>,
{
    #[inline]
    fn log_query(self) -> Self {
        // Only log when in debug mode.
        #[cfg(debug_assertions)]
        log::info!("query: {}", diesel::debug_query::<Sqlite, _>(&self));
        self
    }
}

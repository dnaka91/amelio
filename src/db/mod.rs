//! All database related functionality.

#![allow(clippy::wildcard_imports)]

use anyhow::{Context, Result};
use chrono::NaiveTime;
use diesel::prelude::*;
use diesel::SqliteConnection;
use rocket::fairing::{AdHoc, Fairing};

use self::connection::DbConn;
use self::models::{
    InitCourseEntity, InitTicketEntity, InitUserEntity, MediumInteractiveEntity,
    MediumQuestionaireEntity, MediumRecordingEntity, MediumTextEntity,
};
use crate::hashing::{self, Hasher};
use crate::models::{Category, Priority, Status, TicketType};

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
    create_sample_tickets(conn).context("sample tickets creation failed")?;
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

/// Create several sample tickets for testing purposes.
fn create_sample_tickets(conn: &SqliteConnection) -> Result<()> {
    use crate::db::schema::{
        medium_interactives, medium_questionaires, medium_recordings, medium_texts, tickets,
    };

    if tickets::table.count().get_result::<i64>(conn)? >= 5 {
        return Ok(());
    }

    diesel::insert_into(tickets::table)
        .values(vec![
            &InitTicketEntity {
                type_: TicketType::CourseBook.as_ref(),
                title: "Der Text ist von oben nach unten geschrieben",
                description: "Blah blah blah",
                category: Category::Editorial.as_ref(),
                priority: Priority::Medium.as_ref(),
                status: Status::Open.as_ref(),
                course_id: 1,
                creator_id: 1,
            },
            &InitTicketEntity {
                type_: TicketType::Vodcast.as_ref(),
                title: "Das Video stoppt nach 5 Sekunden",
                description: "Blah blah blah",
                category: Category::Content.as_ref(),
                priority: Priority::Critical.as_ref(),
                status: Status::InProgress.as_ref(),
                course_id: 1,
                creator_id: 1,
            },
            &InitTicketEntity {
                type_: TicketType::InteractiveBook.as_ref(),
                title: "Die Schriftfarbe ist zu grell",
                description: "Blah blah blah",
                category: Category::Improvement.as_ref(),
                priority: Priority::Low.as_ref(),
                status: Status::Accepted.as_ref(),
                course_id: 1,
                creator_id: 1,
            },
            &InitTicketEntity {
                type_: TicketType::PracticeExam.as_ref(),
                title: "Mathematische Formel hat ein falsches Ergebnis",
                description: "Blah blah blah",
                category: Category::Content.as_ref(),
                priority: Priority::High.as_ref(),
                status: Status::Refused.as_ref(),
                course_id: 1,
                creator_id: 1,
            },
            &InitTicketEntity {
                type_: TicketType::Vodcast.as_ref(),
                title: "Der Ton fehlt im gesamten Video",
                description: "Blah blah blah",
                category: Category::Content.as_ref(),
                priority: Priority::High.as_ref(),
                status: Status::Completed.as_ref(),
                course_id: 1,
                creator_id: 1,
            },
        ])
        .execute(&*conn)?;

    diesel::insert_into(medium_texts::table)
        .values(MediumTextEntity {
            ticket_id: 1,
            page: 40,
            line: 3,
        })
        .execute(&*conn)?;

    diesel::insert_into(medium_recordings::table)
        .values(vec![
            MediumRecordingEntity {
                ticket_id: 2,
                time: NaiveTime::from_hms(0, 5, 0).format("%H:%M:%S").to_string(),
            },
            MediumRecordingEntity {
                ticket_id: 5,
                time: NaiveTime::from_hms(2, 12, 55)
                    .format("%H:%M:%S")
                    .to_string(),
            },
        ])
        .execute(&*conn)?;

    diesel::insert_into(medium_interactives::table)
        .values(MediumInteractiveEntity {
            ticket_id: 3,
            url: "https://example.com".to_owned(),
        })
        .execute(&*conn)?;

    diesel::insert_into(medium_questionaires::table)
        .values(MediumQuestionaireEntity {
            ticket_id: 4,
            question: 5,
            answer: "1 + 2 = 4".to_owned(),
        })
        .execute(&*conn)?;

    Ok(())
}

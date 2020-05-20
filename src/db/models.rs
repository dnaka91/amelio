//! Models that map from database rows to `struct`s and back.

use std::convert::{TryFrom, TryInto};

use chrono::NaiveTime;

use super::schema::*;

use crate::models::*;

/// A special new user that is used during first initialization of the database.
#[derive(Insertable)]
#[table_name = "users"]
pub struct InitUserEntity<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub role: &'a str,
    pub active: bool,
}

/// A new user to be added to the database.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserEntity {
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: String,
    pub code: String,
}

impl From<NewUser> for NewUserEntity {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username,
            password: String::new(),
            name: value.name,
            role: value.role.to_string(),
            code: value.code,
        }
    }
}

/// A full user entity equivalent to the `users` table.
#[derive(Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: String,
    pub active: bool,
    pub code: String,
}

impl TryFrom<UserEntity> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            username: value.username,
            password: value.password,
            name: value.name,
            role: value.role.parse()?,
            active: value.active,
            code: value.code,
        })
    }
}

/// A special new course that is used during first initialization of the database.
#[derive(Insertable)]
#[table_name = "courses"]
pub struct InitCourseEntity<'a> {
    pub code: &'a str,
    pub title: &'a str,
    pub author_id: i32,
    pub tutor_id: i32,
    pub active: bool,
}

/// A new course to be added to the database.
#[derive(Insertable)]
#[table_name = "courses"]
pub struct NewCourseEntity {
    pub code: String,
    pub title: String,
    pub author_id: i32,
    pub tutor_id: i32,
}

impl From<NewCourse> for NewCourseEntity {
    fn from(value: NewCourse) -> Self {
        Self {
            code: value.code,
            title: value.title,
            author_id: value.author_id,
            tutor_id: value.tutor_id,
        }
    }
}

/// A full course entity equivalent to the `courses` table.
#[derive(Queryable)]
pub struct CourseEntity {
    pub id: i32,
    pub code: String,
    pub title: String,
    pub author_id: i32,
    pub tutor_id: i32,
    pub active: bool,
}

impl TryFrom<CourseEntity> for Course {
    type Error = anyhow::Error;

    fn try_from(value: CourseEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            code: value.code,
            title: value.title,
            author_id: value.author_id,
            tutor_id: value.tutor_id,
            active: value.active,
        })
    }
}

/// A special new ticket that is used during first initialization of the database.
#[derive(Insertable)]
#[table_name = "tickets"]
pub struct InitTicketEntity<'a> {
    pub type_: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub category: &'a str,
    pub priority: &'a str,
    pub status: &'a str,
    pub course_id: i32,
    pub creator_id: i32,
}

/// A new ticket to be added to the database.
#[derive(Insertable)]
#[table_name = "tickets"]
pub struct NewTicketEntity {
    pub type_: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: String,
    pub course_id: i32,
    pub creator_id: i32,
}

impl From<(NewTicket, Priority)> for NewTicketEntity {
    fn from(value: (NewTicket, Priority)) -> Self {
        Self {
            type_: value.0.type_.to_string(),
            title: value.0.title,
            description: value.0.description,
            category: value.0.category.to_string(),
            priority: value.1.to_string(),
            course_id: value.0.course_id,
            creator_id: value.0.creator_id,
        }
    }
}

/// A full ticket entity equivalent to the `tickets` table.
#[derive(Queryable)]
pub struct TicketEntity {
    pub id: i32,
    pub type_: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub course_id: i32,
    pub creator_id: i32,
}

impl TryFrom<TicketEntity> for Ticket {
    type Error = anyhow::Error;

    fn try_from(value: TicketEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            type_: value.type_.parse()?,
            title: value.title,
            description: value.description,
            category: value.category.parse()?,
            priority: value.priority.parse()?,
            status: value.status.parse()?,
            course_id: value.course_id,
            creator_id: value.creator_id,
        })
    }
}

/// A full text medium entity equivalent to the`medium_texts` table, also representing a new entry
/// that can be added to the system.
#[derive(Queryable, Insertable)]
#[table_name = "medium_texts"]
pub struct MediumTextEntity {
    pub ticket_id: i32,
    pub page: i32,
    pub line: i32,
}

impl TryFrom<MediumTextEntity> for Medium {
    type Error = anyhow::Error;

    fn try_from(value: MediumTextEntity) -> Result<Self, Self::Error> {
        Ok(Self::Text {
            ticket_id: value.ticket_id,
            page: value.page.try_into()?,
            line: value.line.try_into()?,
        })
    }
}

/// A full recording medium entity equivalent to the`medium_recordings` table, also representing a
/// new entry that can be added to the system.
#[derive(Queryable, Insertable)]
#[table_name = "medium_recordings"]
pub struct MediumRecordingEntity {
    pub ticket_id: i32,
    pub time: String,
}

impl TryFrom<MediumRecordingEntity> for Medium {
    type Error = anyhow::Error;

    fn try_from(value: MediumRecordingEntity) -> Result<Self, Self::Error> {
        Ok(Self::Recording {
            ticket_id: value.ticket_id,
            time: NaiveTime::parse_from_str(&value.time, "%H:%M:%S")?,
        })
    }
}

/// A full interactive medium entity equivalent to the`medium_interactives` table, also representing
/// a new entry that can be added to the system.
#[derive(Queryable, Insertable)]
#[table_name = "medium_interactives"]
pub struct MediumInteractiveEntity {
    pub ticket_id: i32,
    pub url: String,
}

impl TryFrom<MediumInteractiveEntity> for Medium {
    type Error = anyhow::Error;

    fn try_from(value: MediumInteractiveEntity) -> Result<Self, Self::Error> {
        Ok(Self::Interactive {
            ticket_id: value.ticket_id,
            url: value.url,
        })
    }
}

/// A full questionaire medium entity equivalent to the`medium_questionaires` table, also
/// representing a new entry that can be added to the system.
#[derive(Queryable, Insertable)]
#[table_name = "medium_questionaires"]
pub struct MediumQuestionaireEntity {
    pub ticket_id: i32,
    pub question: i32,
    pub answer: String,
}

impl TryFrom<MediumQuestionaireEntity> for Medium {
    type Error = anyhow::Error;

    fn try_from(value: MediumQuestionaireEntity) -> Result<Self, Self::Error> {
        Ok(Self::Questionaire {
            ticket_id: value.ticket_id,
            question: value.question.try_into()?,
            answer: value.answer,
        })
    }
}

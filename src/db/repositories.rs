//! Abstractions over the database for easy access to the data.

use std::convert::TryInto;
use std::iter::FromIterator;

use anyhow::{ensure, Context, Result};
use diesel::prelude::*;
use fnv::{FnvHashMap, FnvHashSet};

use super::models::{
    CourseEntity, MediumInteractiveEntity, MediumQuestionaireEntity, MediumRecordingEntity,
    MediumTextEntity, NewCourseEntity, NewTicketEntity, NewUserEntity, TicketEntity, UserEntity,
};

use crate::models::{
    Course, CourseWithNames, EditCourse, EditUser, MediumType, NewCourse, NewMedium, NewTicket,
    NewUser, Priority, Role, Ticket, TicketWithMedium, TicketWithNames, User,
};

/// User related functionality.
pub trait UserRepository {
    /// Find a single user by its ID.
    fn find(&self, id: i32) -> Result<User>;
    /// Find a singel user by its username.
    fn find_by_username(&self, username: &str) -> Result<User>;
    /// List all users.
    fn list(&self) -> Result<Vec<User>>;
    /// List all users' ID and name filtered by role.
    fn list_names_by_role(&self, role: Role) -> Result<Vec<(i32, String)>>;
    /// Create a new user.
    fn create(&self, user: NewUser) -> Result<()>;
    /// Activate a previously created user.
    fn activate(&self, code: &str, password: &str) -> Result<()>;
    /// Enable or disable an existing user.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
    /// Update an existing user.
    fn update(&self, user: EditUser) -> Result<()>;
}

/// Main implementation of [`UserRepository`].
struct UserRepositoryImpl<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> UserRepository for UserRepositoryImpl<'a> {
    fn find(&self, id: i32) -> Result<User> {
        use super::schema::users;

        users::table
            .find(id)
            .filter(users::active.eq(true))
            .get_result::<UserEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn find_by_username(&self, username: &str) -> Result<User> {
        use super::schema::users;

        users::table
            .filter(users::active.eq(true).and(users::username.eq(username)))
            .get_result::<UserEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn list(&self) -> Result<Vec<User>> {
        use super::schema::users;

        users::table
            .load::<UserEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|users| users.into_iter().map(TryInto::try_into).collect())
    }

    fn list_names_by_role(&self, role: Role) -> Result<Vec<(i32, String)>> {
        use super::schema::users;

        users::table
            .select((users::id, users::name))
            .filter(users::role.eq(role.as_ref()))
            .load::<(i32, String)>(self.conn)
            .map_err(Into::into)
    }

    fn create(&self, user: NewUser) -> Result<()> {
        use super::schema::users;

        let res = diesel::insert_into(users::table)
            .values(NewUserEntity::from(user))
            .execute(self.conn)?;

        ensure!(res == 1, "Failed inserting user");
        Ok(())
    }

    fn activate(&self, code: &str, password: &str) -> Result<()> {
        use super::schema::users;

        let res = diesel::update(users::table.filter(users::code.eq(code)))
            .set((
                users::password.eq(password),
                users::active.eq(true),
                users::code.eq(""),
            ))
            .execute(self.conn)?;

        ensure!(res == 1, "User with code {} not found", code);
        Ok(())
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        use super::schema::users;

        let res = diesel::update(users::table.filter(users::id.eq(id)))
            .set(users::active.eq(enable))
            .execute(self.conn)?;

        ensure!(res == 1, "User with ID {} not found", id);
        Ok(())
    }

    fn update(&self, user: EditUser) -> Result<()> {
        use super::schema::users;

        let res = diesel::update(users::table.filter(users::id.eq(user.id)))
            .set((
                users::name.eq(user.name),
                users::role.eq(user.role.as_ref()),
            ))
            .execute(self.conn)?;

        ensure!(res == 1, "User with ID {} not found", user.id);
        Ok(())
    }
}

/// Create a new user repository.
pub fn user_repo<'a>(conn: &'a SqliteConnection) -> impl UserRepository + 'a {
    UserRepositoryImpl { conn }
}

/// Course related functionality.
pub trait CourseRepository {
    /// List all courses together with their author and tutor names.
    fn list_with_names(&self) -> Result<Vec<CourseWithNames>>;
    /// List the names (with IDs) of all users that can be assigned to courses.
    fn list_names(&self) -> Result<Vec<(i32, String)>>;
    /// Get a single course by ID.
    fn get(&self, id: i32) -> Result<Course>;
    /// Create a new course.
    fn create(&self, course: NewCourse) -> Result<()>;
    /// Enable or disable an existing course.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
    /// Update an existing course.
    fn update(&self, course: EditCourse) -> Result<()>;
}

/// Main implementation of [`CourseRepository`].
struct CourseRepositoryImpl<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> CourseRepositoryImpl<'a> {
    /// List all courses.
    fn list(&self) -> Result<Vec<Course>> {
        use super::schema::courses;

        courses::table
            .order_by(courses::code)
            .load::<CourseEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|courses| courses.into_iter().map(TryInto::try_into).collect())
    }
}

impl<'a> CourseRepository for CourseRepositoryImpl<'a> {
    fn list_with_names(&self) -> Result<Vec<CourseWithNames>> {
        use super::schema::users;

        let courses = self.list()?;
        let mut user_ids = FnvHashSet::default();

        for course in &courses {
            user_ids.insert(course.author_id);
            user_ids.insert(course.tutor_id);
        }

        let users = users::table
            .select((users::id, users::name))
            .filter(users::id.eq_any(&user_ids))
            .load::<(i32, String)>(self.conn)
            .map(FnvHashMap::from_iter)?;

        courses
            .into_iter()
            .map(|course| {
                let author_name = users
                    .get(&course.author_id)
                    .cloned()
                    .context("Entry missing for course's author ID")?;
                let tutor_name = users
                    .get(&course.tutor_id)
                    .cloned()
                    .context("Entry missing for course's tutor ID")?;
                Ok(CourseWithNames {
                    course,
                    author_name,
                    tutor_name,
                })
            })
            .collect()
    }

    fn list_names(&self) -> Result<Vec<(i32, String)>> {
        use super::schema::courses;

        courses::table
            .select((courses::id, courses::code))
            .order_by(courses::code)
            .load::<(i32, String)>(self.conn)
            .map_err(Into::into)
    }

    fn get(&self, id: i32) -> Result<Course> {
        use super::schema::courses;

        courses::table
            .find(id)
            .get_result::<CourseEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn create(&self, course: NewCourse) -> Result<()> {
        use super::schema::courses;

        let res = diesel::insert_into(courses::table)
            .values(NewCourseEntity::from(course))
            .execute(self.conn)?;

        ensure!(res == 1, "Failed inserting course");
        Ok(())
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        use super::schema::courses;

        let res = diesel::update(courses::table.filter(courses::id.eq(id)))
            .set(courses::active.eq(enable))
            .execute(self.conn)?;

        ensure!(res == 1, "Course with ID {} not found", id);
        Ok(())
    }

    fn update(&self, course: EditCourse) -> Result<()> {
        use super::schema::courses;

        let res = diesel::update(courses::table.filter(courses::id.eq(course.id)))
            .set((
                courses::title.eq(course.title),
                courses::author_id.eq(course.author_id),
                courses::tutor_id.eq(course.tutor_id),
            ))
            .execute(self.conn)?;

        ensure!(res == 1, "Course with ID {} not found", course.id);
        Ok(())
    }
}

/// Create a new course repository.
pub fn course_repo<'a>(conn: &'a SqliteConnection) -> impl CourseRepository + 'a {
    CourseRepositoryImpl { conn }
}

/// Ticket related functionality.
pub trait TicketRepository {
    /// List all tickets together with their course and creator names.
    fn list_with_names(&self) -> Result<Vec<TicketWithNames>>;
    /// Get a single ticket with course and creator names.
    fn get_with_names(&self, id: i32) -> Result<TicketWithNames>;
    /// Get a single ticket with names and attached medium.
    fn get_with_medium(&self, id: i32) -> Result<TicketWithMedium>;
    /// Create a new ticket.
    fn create(&self, ticket: NewTicket, priority: Priority, medium: NewMedium) -> Result<()>;
}

/// Main implementation of [`TicketRepository`].
struct TicketRepositoryImpl<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> TicketRepositoryImpl<'a> {
    /// List all tickets.
    fn list(&self) -> Result<Vec<Ticket>> {
        use super::schema::tickets;

        tickets::table
            .load::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|users| users.into_iter().map(TryInto::try_into).collect())
    }

    /// Get a single ticket by ID.
    fn get(&self, id: i32) -> Result<Ticket> {
        use super::schema::tickets;

        tickets::table
            .find(id)
            .get_result::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }
}

impl<'a> TicketRepository for TicketRepositoryImpl<'a> {
    fn list_with_names(&self) -> Result<Vec<TicketWithNames>> {
        use super::schema::{courses, users};

        let tickets = self.list()?;
        let mut user_ids = FnvHashSet::default();
        let mut course_ids = FnvHashSet::default();

        for ticket in &tickets {
            user_ids.insert(ticket.creator_id);
            course_ids.insert(ticket.course_id);
        }

        let courses = courses::table
            .select((courses::id, courses::code))
            .filter(courses::id.eq_any(&course_ids))
            .load::<(i32, String)>(self.conn)
            .map(FnvHashMap::from_iter)?;

        let users = users::table
            .select((users::id, users::name))
            .filter(users::id.eq_any(&user_ids))
            .load::<(i32, String)>(self.conn)
            .map(FnvHashMap::from_iter)?;

        tickets
            .into_iter()
            .map(|ticket| {
                let course_name = courses
                    .get(&ticket.course_id)
                    .cloned()
                    .context("Entry missing for tickets's course ID")?;
                let creator_name = users
                    .get(&ticket.creator_id)
                    .cloned()
                    .context("Entry missing for tickets's creator ID")?;
                Ok(TicketWithNames {
                    ticket,
                    creator_name,
                    course_name,
                })
            })
            .collect()
    }

    fn get_with_names(&self, id: i32) -> Result<TicketWithNames> {
        use super::schema::{courses, users};

        let ticket = self.get(id)?;

        let course_name = courses::table
            .find(ticket.course_id)
            .select(courses::code)
            .get_result(self.conn)?;

        let creator_name = users::table
            .find(ticket.creator_id)
            .select(users::name)
            .get_result(self.conn)?;

        Ok(TicketWithNames {
            ticket,
            creator_name,
            course_name,
        })
    }

    fn get_with_medium(&self, id: i32) -> Result<TicketWithMedium> {
        use super::schema::{
            medium_interactives, medium_questionaires, medium_recordings, medium_texts,
        };

        let ticket = self.get_with_names(id)?;

        let medium = match ticket.ticket.type_.medium() {
            MediumType::Text => medium_texts::table
                .find(id)
                .get_result::<MediumTextEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Recording => medium_recordings::table
                .find(id)
                .get_result::<MediumRecordingEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Interactive => medium_interactives::table
                .find(id)
                .get_result::<MediumInteractiveEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Questionaire => medium_questionaires::table
                .find(id)
                .get_result::<MediumQuestionaireEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
        }?;

        Ok(TicketWithMedium {
            ticket: ticket.ticket,
            course_name: ticket.course_name,
            creator_name: ticket.creator_name,
            medium,
        })
    }

    fn create(&self, ticket: NewTicket, priority: Priority, medium: NewMedium) -> Result<()> {
        use super::schema::{
            medium_interactives, medium_questionaires, medium_recordings, medium_texts, tickets,
        };

        self.conn.transaction(|| {
            let res = diesel::insert_into(tickets::table)
                .values(NewTicketEntity::from((ticket, priority)))
                .execute(self.conn)?;

            ensure!(res == 1, "Failed inserting ticket");

            let ticket_id = tickets::table
                .select(tickets::id)
                .order_by(tickets::id.desc())
                .limit(1)
                .get_result::<i32>(self.conn)?;

            let res_medium = match medium {
                NewMedium::Text { page, line } => diesel::insert_into(medium_texts::table)
                    .values(MediumTextEntity {
                        ticket_id,
                        page: page.into(),
                        line: line.into(),
                    })
                    .execute(self.conn),
                NewMedium::Recording { time } => diesel::insert_into(medium_recordings::table)
                    .values(MediumRecordingEntity {
                        ticket_id,
                        time: time.format("%H:%M:%S").to_string(),
                    })
                    .execute(self.conn),
                NewMedium::Interactive { url } => diesel::insert_into(medium_interactives::table)
                    .values(MediumInteractiveEntity { ticket_id, url })
                    .execute(self.conn),
                NewMedium::Questionaire { question, answer } => {
                    diesel::insert_into(medium_questionaires::table)
                        .values(MediumQuestionaireEntity {
                            ticket_id,
                            question: question.into(),
                            answer,
                        })
                        .execute(self.conn)
                }
            }?;

            ensure!(res_medium == 1, "Failed inserting medium");
            Ok(())
        })
    }
}

/// Create a new ticket repository.
pub fn ticket_repo<'a>(conn: &'a SqliteConnection) -> impl TicketRepository + 'a {
    TicketRepositoryImpl { conn }
}

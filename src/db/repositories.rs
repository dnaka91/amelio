//! Abstractions over the database for easy access to the data.

use std::convert::TryInto;
use std::iter::FromIterator;

use anyhow::{ensure, Context, Result};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use fnv::{FnvHashMap, FnvHashSet};

use super::models::{
    CommentEntity, CourseEntity, MediumInteractiveEntity, MediumQuestionaireEntity,
    MediumRecordingEntity, MediumTextEntity, NewCommentEntity, NewCourseEntity, NewTicketEntity,
    NewUserEntity, TicketEntity, UserEntity,
};
use super::QueryExt;
use crate::models::{
    Comment, CommentWithNames, Course, CourseWithNames, EditCourse, EditTicket, EditUser,
    MediumType, NewComment, NewCourse, NewMedium, NewTicket, NewUser, Priority, Role, Status,
    Ticket, TicketSearch, TicketWithNames, TicketWithRels, User,
};

/// User related functionality.
pub trait UserRepository {
    /// Find a single user by its ID.
    fn find(&self, id: i32) -> Result<User>;
    /// Find a single user by its username.
    fn find_by_username(&self, username: &str) -> Result<User>;
    /// Find the user who created a ticket.
    fn find_ticket_creator(&self, ticket_id: i32) -> Result<User>;
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
            .log_query()
            .get_result::<UserEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn find_by_username(&self, username: &str) -> Result<User> {
        use super::schema::users;

        users::table
            .filter(users::active.eq(true).and(users::username.eq(username)))
            .log_query()
            .get_result::<UserEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn find_ticket_creator(&self, ticket_id: i32) -> Result<User> {
        use super::schema::{tickets, users};

        tickets::table
            .find(ticket_id)
            .inner_join(users::table)
            .select(users::all_columns)
            .log_query()
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
            .log_query()
            .load::<(i32, String)>(self.conn)
            .map_err(Into::into)
    }

    fn create(&self, user: NewUser) -> Result<()> {
        use super::schema::users;

        let res = diesel::insert_into(users::table)
            .values(NewUserEntity::from(user))
            .log_query()
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
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "User with code {} not found", code);
        Ok(())
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        use super::schema::users;

        let res = diesel::update(users::table.filter(users::id.eq(id)))
            .set(users::active.eq(enable))
            .log_query()
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
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "User with ID {} not found", user.id);
        Ok(())
    }
}

/// Create a new user repository.
pub fn user_repo(conn: &SqliteConnection) -> impl UserRepository + '_ {
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
            .log_query()
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
            .log_query()
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
            .log_query()
            .load::<(i32, String)>(self.conn)
            .map_err(Into::into)
    }

    fn get(&self, id: i32) -> Result<Course> {
        use super::schema::courses;

        courses::table
            .find(id)
            .log_query()
            .get_result::<CourseEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn create(&self, course: NewCourse) -> Result<()> {
        use super::schema::courses;

        let res = diesel::insert_into(courses::table)
            .values(NewCourseEntity::from(course))
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Failed inserting course");
        Ok(())
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        use super::schema::courses;

        let res = diesel::update(courses::table.filter(courses::id.eq(id)))
            .set(courses::active.eq(enable))
            .log_query()
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
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Course with ID {} not found", course.id);
        Ok(())
    }
}

/// Create a new course repository.
pub fn course_repo(conn: &SqliteConnection) -> impl CourseRepository + '_ {
    CourseRepositoryImpl { conn }
}

/// Ticket related functionality.
pub trait TicketRepository {
    /// List all tickets together with their course and creator names.
    fn list_with_names(&self) -> Result<Vec<TicketWithNames>>;
    /// List all tickets by their creator ID.
    fn list_by_creator_id(&self, creator_id: i32) -> Result<Vec<TicketWithNames>>;
    /// List all tickets by their assignee ID. The assignee ID is the tutor or author ID of the
    /// course that a ticket belongs to.
    fn list_by_assignee_id(&self, assignee_id: i32) -> Result<Vec<TicketWithNames>>;
    /// Get a single ticket by ID.
    fn get(&self, id: i32) -> Result<Ticket>;
    /// Get a single ticket with course and creator names.
    fn get_with_names(&self, id: i32) -> Result<TicketWithNames>;
    /// Get a single ticket with all related data.
    fn get_with_rels(&self, id: i32) -> Result<TicketWithRels>;
    /// Create a new ticket.
    fn create(&self, ticket: NewTicket, priority: Priority, medium: NewMedium) -> Result<i32>;
    /// Add a new comment to an existing ticket.
    fn add_comment(&self, comment: NewComment) -> Result<()>;
    /// Update an existing ticket.
    fn update(&self, ticket: EditTicket) -> Result<()>;
    /// Forward a ticket to its course's author.
    fn forward(&self, id: i32) -> Result<()>;
    /// Get the current status of a ticket.
    fn get_status(&self, id: i32) -> Result<Status>;
    /// Set the new status of a ticket.
    fn set_status(&self, id: i32, status: Status) -> Result<()>;
    /// Search for tickets with different criteria.
    fn search(&self, search: &TicketSearch) -> Result<Vec<TicketWithNames>>;
    /// Activate a new ticket, changing it to [`Status::InProgress`] if it's still in
    /// [`Status::Open`] and accessed by a tutor or author user.
    fn activate_ticket(&self, id: i32, user_id: i32) -> Result<bool>;
    /// Check whether the provided user is the creator of a ticket.
    fn is_creator(&self, id: i32, user_id: i32) -> Result<bool>;
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
            .and_then(|entities| entities.into_iter().map(TryInto::try_into).collect())
    }

    /// Load user and course names and attach them to the given list of tickets.
    fn load_names(&self, tickets: Vec<Ticket>) -> Result<Vec<TicketWithNames>> {
        use super::schema::{courses, users};

        let mut user_ids = FnvHashSet::default();
        let mut course_ids = FnvHashSet::default();

        for ticket in &tickets {
            user_ids.insert(ticket.creator_id);
            course_ids.insert(ticket.course_id);
        }

        let courses = courses::table
            .select((
                courses::id,
                courses::code,
                courses::author_id,
                courses::tutor_id,
            ))
            .filter(courses::id.eq_any(&course_ids))
            .log_query()
            .load::<(i32, String, i32, i32)>(self.conn)
            .map(|data| {
                data.into_iter()
                    .map(|row| (row.0, (row.1, row.2, row.3)))
                    .collect::<FnvHashMap<_, _>>()
            })?;

        for (_, author_id, tutor_id) in courses.values() {
            user_ids.insert(*author_id);
            user_ids.insert(*tutor_id);
        }

        let users = users::table
            .select((users::id, users::name))
            .filter(users::id.eq_any(&user_ids))
            .log_query()
            .load::<(i32, String)>(self.conn)
            .map(FnvHashMap::from_iter)?;

        tickets
            .into_iter()
            .map(|ticket| {
                let (course_name, course_author, course_tutor) = courses
                    .get(&ticket.course_id)
                    .cloned()
                    .context("Entry missing for tickets's course ID")?;
                let creator_name = users
                    .get(&ticket.creator_id)
                    .cloned()
                    .context("Entry missing for tickets's creator ID")?;
                let editor_name = users
                    .get(&if ticket.forwarded {
                        course_author
                    } else {
                        course_tutor
                    })
                    .cloned()
                    .context("Entry missing for ticket's editor ID")?;
                Ok(TicketWithNames {
                    ticket,
                    course_name,
                    creator_name,
                    editor_name,
                })
            })
            .collect()
    }
}

impl<'a> TicketRepository for TicketRepositoryImpl<'a> {
    fn list_with_names(&self) -> Result<Vec<TicketWithNames>> {
        let tickets = self.list()?;

        self.load_names(tickets)
    }

    fn list_by_creator_id(&self, creator_id: i32) -> Result<Vec<TicketWithNames>> {
        use super::schema::tickets;

        let tickets = tickets::table
            .filter(tickets::creator_id.eq(creator_id))
            .log_query()
            .load::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|entities| entities.into_iter().map(TryInto::try_into).collect())?;

        self.load_names(tickets)
    }

    fn list_by_assignee_id(&self, assignee_id: i32) -> Result<Vec<TicketWithNames>> {
        use super::schema::{courses, tickets};

        let tickets = tickets::table
            .inner_join(courses::table)
            .filter(
                courses::author_id
                    .eq(assignee_id)
                    .and(tickets::forwarded.eq(true))
                    .or(courses::tutor_id
                        .eq(assignee_id)
                        .and(tickets::forwarded.eq(false))),
            )
            .select(tickets::all_columns)
            .log_query()
            .load::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|entities| entities.into_iter().map(TryInto::try_into).collect())?;

        self.load_names(tickets)
    }

    fn get(&self, id: i32) -> Result<Ticket> {
        use super::schema::tickets;

        tickets::table
            .find(id)
            .log_query()
            .get_result::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(TryInto::try_into)
    }

    fn get_with_names(&self, id: i32) -> Result<TicketWithNames> {
        use super::schema::{courses, users};

        let ticket = self.get(id)?;

        let (course_name, author_id, tutor_id) = courses::table
            .find(ticket.course_id)
            .select((courses::code, courses::author_id, courses::tutor_id))
            .log_query()
            .get_result::<(String, i32, i32)>(self.conn)?;

        let creator_name = users::table
            .find(ticket.creator_id)
            .select(users::name)
            .log_query()
            .get_result(self.conn)?;

        let editor_name = users::table
            .find(if ticket.forwarded {
                author_id
            } else {
                tutor_id
            })
            .select(users::name)
            .log_query()
            .get_result(self.conn)?;

        Ok(TicketWithNames {
            ticket,
            course_name,
            creator_name,
            editor_name,
        })
    }

    fn get_with_rels(&self, id: i32) -> Result<TicketWithRels> {
        use super::schema::{
            comments, medium_interactives, medium_questionaires, medium_recordings, medium_texts,
            users,
        };

        let ticket = self.get_with_names(id)?;

        let medium = match ticket.ticket.type_.medium() {
            MediumType::Text => medium_texts::table
                .find(id)
                .log_query()
                .get_result::<MediumTextEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Recording => medium_recordings::table
                .find(id)
                .log_query()
                .get_result::<MediumRecordingEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Interactive => medium_interactives::table
                .find(id)
                .log_query()
                .get_result::<MediumInteractiveEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
            MediumType::Questionaire => medium_questionaires::table
                .find(id)
                .log_query()
                .get_result::<MediumQuestionaireEntity>(self.conn)
                .map_err(Into::into)
                .and_then(TryInto::try_into),
        }?;

        let comments = comments::table
            .filter(comments::ticket_id.eq(id))
            .log_query()
            .load::<CommentEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|comments| {
                comments
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<Comment>>>()
            })?;

        let creator_ids = comments
            .iter()
            .map(|c| c.creator_id)
            .collect::<FnvHashSet<_>>();

        let users = users::table
            .select((users::id, users::name))
            .filter(users::id.eq_any(creator_ids))
            .log_query()
            .load::<(i32, String)>(self.conn)
            .map(FnvHashMap::from_iter)?;

        let comments = comments
            .into_iter()
            .map(|comment| {
                let creator_name = users
                    .get(&comment.creator_id)
                    .cloned()
                    .context("Entry missing for comment's user ID")?;

                Ok(CommentWithNames {
                    comment,
                    creator_name,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(TicketWithRels {
            ticket: ticket.ticket,
            course_name: ticket.course_name,
            creator_name: ticket.creator_name,
            editor_name: ticket.editor_name,
            medium,
            comments,
        })
    }

    fn create(&self, ticket: NewTicket, priority: Priority, medium: NewMedium) -> Result<i32> {
        use super::schema::{
            medium_interactives, medium_questionaires, medium_recordings, medium_texts, tickets,
        };

        self.conn.transaction(|| {
            let res = diesel::insert_into(tickets::table)
                .values(NewTicketEntity::from((ticket, priority)))
                .log_query()
                .execute(self.conn)?;

            ensure!(res == 1, "Failed inserting ticket");

            let ticket_id = tickets::table
                .select(tickets::id)
                .order_by(tickets::id.desc())
                .limit(1)
                .log_query()
                .get_result::<i32>(self.conn)?;

            let res_medium = match medium {
                NewMedium::Text { page, line } => diesel::insert_into(medium_texts::table)
                    .values(MediumTextEntity {
                        ticket_id,
                        page: page.into(),
                        line: line.into(),
                    })
                    .log_query()
                    .execute(self.conn),
                NewMedium::Recording { time } => diesel::insert_into(medium_recordings::table)
                    .values(MediumRecordingEntity {
                        ticket_id,
                        time: time.format("%H:%M:%S").to_string(),
                    })
                    .log_query()
                    .execute(self.conn),
                NewMedium::Interactive { url } => diesel::insert_into(medium_interactives::table)
                    .values(MediumInteractiveEntity {
                        ticket_id,
                        url: url.into_string(),
                    })
                    .log_query()
                    .execute(self.conn),
                NewMedium::Questionaire { question, answer } => {
                    diesel::insert_into(medium_questionaires::table)
                        .values(MediumQuestionaireEntity {
                            ticket_id,
                            question: question.into(),
                            answer,
                        })
                        .log_query()
                        .execute(self.conn)
                }
            }?;

            ensure!(res_medium == 1, "Failed inserting medium");
            Ok(ticket_id)
        })
    }

    fn add_comment(&self, comment: NewComment) -> Result<()> {
        use super::schema::comments;

        let res = diesel::insert_into(comments::table)
            .values(NewCommentEntity::from(comment))
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Failed inserting comment");
        Ok(())
    }

    fn update(&self, ticket: EditTicket) -> Result<()> {
        use super::schema::tickets;

        let res = diesel::update(tickets::table.find(ticket.id))
            .set(tickets::priority.eq(ticket.priority.as_ref()))
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Ticket with ID {} not found", ticket.id);
        Ok(())
    }

    fn forward(&self, id: i32) -> Result<()> {
        use super::schema::tickets;

        let res = diesel::update(tickets::table.find(id))
            .set(tickets::forwarded.eq(true))
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Ticket with ID {} not found", id);
        Ok(())
    }

    fn get_status(&self, id: i32) -> Result<Status> {
        use super::schema::tickets;

        tickets::table
            .find(id)
            .select(tickets::status)
            .log_query()
            .get_result::<String>(self.conn)
            .map_err(Into::into)
            .and_then(|v| v.parse().map_err(Into::into))
    }

    fn set_status(&self, id: i32, status: Status) -> Result<()> {
        use super::schema::tickets;

        let res = diesel::update(tickets::table.find(id))
            .set(tickets::status.eq(status.as_ref()))
            .log_query()
            .execute(self.conn)?;

        ensure!(res == 1, "Ticket with ID {} not found", id);
        Ok(())
    }

    fn search(&self, search: &TicketSearch) -> Result<Vec<TicketWithNames>> {
        use super::schema::tickets;

        let mut query = tickets::table.into_boxed();

        if let Some(title) = &search.title {
            query = query.filter(tickets::title.like(format!("%{}%", title)));
        }

        if let Some(course_id) = search.course_id {
            query = query.filter(tickets::course_id.eq(course_id));
        }

        if let Some(category) = search.category {
            query = query.filter(tickets::category.eq(category.to_string()));
        }

        if let Some(priority) = search.priority {
            query = query.filter(tickets::priority.eq(priority.to_string()));
        }

        if let Some(status) = search.status {
            query = query.filter(tickets::status.eq(status.to_string()));
        }

        let tickets = query
            .log_query()
            .load::<TicketEntity>(self.conn)
            .map_err(Into::into)
            .and_then(|entities| entities.into_iter().map(TryInto::try_into).collect())?;

        self.load_names(tickets)
    }

    fn activate_ticket(&self, id: i32, user_id: i32) -> Result<bool> {
        use super::schema::{courses, tickets};

        let res = tickets::table
            .find(id)
            .select(tickets::id)
            .inner_join(courses::table)
            .filter(tickets::status.eq(Status::Open.as_ref()))
            .filter(
                courses::tutor_id
                    .eq(user_id)
                    .and(tickets::forwarded.eq(false))
                    .or(courses::author_id
                        .eq(user_id)
                        .and(tickets::forwarded.eq(true))),
            )
            .log_query()
            .get_result::<i32>(self.conn);

        let found = match res {
            Ok(_) => true,
            Err(DieselError::NotFound) => false,
            Err(e) => return Err(e.into()),
        };

        if found {
            let res = diesel::update(tickets::table.find(id))
                .set(tickets::status.eq(Status::InProgress.as_ref()))
                .log_query()
                .execute(self.conn)?;

            ensure!(res == 1, "Failed opening ticket");
            return Ok(true);
        }

        Ok(false)
    }

    fn is_creator(&self, id: i32, user_id: i32) -> Result<bool> {
        use super::schema::tickets;
        use diesel::dsl::count;

        let res = tickets::table
            .find(id)
            .select(count(tickets::id))
            .filter(tickets::creator_id.eq(user_id))
            .log_query()
            .get_result::<i64>(self.conn)?;

        Ok(res == 1)
    }
}

/// Create a new ticket repository.
pub fn ticket_repo(conn: &SqliteConnection) -> impl TicketRepository + '_ {
    TicketRepositoryImpl { conn }
}

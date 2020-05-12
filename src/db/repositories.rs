//! Abstractions over the database for easy access to the data.

use std::convert::TryInto;
use std::iter::FromIterator;

use anyhow::{ensure, Context, Result};
use diesel::prelude::*;
use fnv::{FnvHashMap, FnvHashSet};

use super::models::{CourseEntity, NewCourseEntity, NewUserEntity, UserEntity};

use crate::models::{Course, CourseWithNames, NewCourse, NewUser, User};

/// User related functionality.
pub trait UserRepository {
    /// Find a single user by its ID.
    fn find(&self, id: i32) -> Result<User>;
    /// Find a singel user by its username.
    fn find_by_username(&self, username: &str) -> Result<User>;
    /// List all users.
    fn list(&self) -> Result<Vec<User>>;
    /// Create a new user.
    fn create(&self, user: NewUser) -> Result<()>;
    /// Activate a previously created user.
    fn activate(&self, code: &str, password: &str) -> Result<()>;
    /// Enable or disable an existing user.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
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
}

/// Create a new user repository.
pub fn user_repo<'a>(conn: &'a SqliteConnection) -> impl UserRepository + 'a {
    UserRepositoryImpl { conn }
}

/// Course related functionality.
pub trait CourseRepository {
    /// List all courses together with their author and tutor names.
    fn list_with_names(&self) -> Result<Vec<CourseWithNames>>;
    /// Create a new course.
    fn create(&self, course: NewCourse) -> Result<()>;
    /// Enable or disable an existing course.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
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
            .load::<(i32, String)>(self.conn)?;

        let users = FnvHashMap::from_iter(users);

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
}

pub fn course_repo<'a>(conn: &'a SqliteConnection) -> impl CourseRepository + 'a {
    CourseRepositoryImpl { conn }
}

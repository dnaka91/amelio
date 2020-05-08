//! Abstractions over the database for easy access to the data.

use std::convert::TryInto;

use anyhow::Result;
use diesel::prelude::*;

use super::models::{NewUserEntity, UserEntity};

use crate::models::{NewUser, User};

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
    fn activate(&self, code: &str, password: &str) -> Result<usize>;
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
            .and_then(|users| users.into_iter().map(|u| u.try_into()).collect())
    }

    fn create(&self, user: NewUser) -> Result<()> {
        use super::schema::users;

        diesel::insert_into(users::table)
            .values(NewUserEntity::from(user))
            .execute(self.conn)?;

        Ok(())
    }

    fn activate(&self, code: &str, password: &str) -> Result<usize> {
        use super::schema::users;

        diesel::update(users::table.filter(users::code.eq(code)))
            .set((
                users::password.eq(password),
                users::active.eq(true),
                users::code.eq(""),
            ))
            .execute(self.conn)
            .map_err(Into::into)
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        use super::schema::users;

        diesel::update(users::table.filter(users::id.eq(id)))
            .set(users::active.eq(enable))
            .execute(self.conn)?;

        Ok(())
    }
}

/// Create a new user repository.
pub fn user_repo<'a>(conn: &'a SqliteConnection) -> impl UserRepository + 'a {
    UserRepositoryImpl { conn }
}

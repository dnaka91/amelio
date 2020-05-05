//! Abstractions over the database for easy access to the data.

use anyhow::Result;
use diesel::prelude::*;

use super::models::User;

/// User related functionality.
pub trait UserRepository {
    /// Find a single user by its ID.
    fn find(&self, id: i32) -> Result<User>;
    /// Find a singel user by its username.
    fn find_by_username(&self, username: &str) -> Result<User>;
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
            .get_result(self.conn)
            .map_err(Into::into)
    }

    fn find_by_username(&self, username: &str) -> Result<User> {
        use super::schema::users;

        users::table
            .filter(users::username.eq(username))
            .get_result(self.conn)
            .map_err(Into::into)
    }
}

/// Create a new user repository.
pub fn user_repo<'a>(conn: &'a SqliteConnection) -> impl UserRepository + 'a {
    UserRepositoryImpl { conn }
}

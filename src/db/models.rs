//! Models that map from database rows to `struct`s and back.

use super::schema::*;

/// A new user to be added to the database.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub name: &'a str,
    pub role: &'a str,
}

/// A full user entity equivalent to the `users` table.
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: String,
}

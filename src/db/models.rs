//! Models that map from database rows to `struct`s and back.

use std::convert::TryFrom;

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

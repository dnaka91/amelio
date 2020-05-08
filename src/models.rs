//! The base models of the system, that [`services`](crate::services) work on.

use strum::{Display, EnumString};

/// The identifier type for all models.
pub type Id = i32;

/// Different roles that a user can have.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Role {
    Admin,
    Author,
    Tutor,
    Student,
}

/// A full user with all available details.
pub struct User {
    pub id: Id,
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: Role,
    pub active: bool,
    pub code: String,
}

impl User {
    /// Check whether this user is the very first administrator.
    pub fn is_admin(&self) -> bool {
        self.id == 1 && self.role == Role::Admin
    }
}

/// A basic new user that is not part of the system yet.
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub role: Role,
    pub code: String,
}

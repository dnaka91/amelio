//! All routes for the server.

#![cfg_attr(doc, allow(unused_braces))]

use rocket::response::Redirect;
use rocket::{get, uri};

use crate::roles::AuthUser;
use crate::templates;

pub mod assets;
pub mod auth;

/// Index page for authenticated users.
#[get("/")]
pub const fn index_user(_user: &AuthUser) -> templates::Index {
    templates::Index
}

/// Index page for non-authenticated users, redirecting to the login page.
#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(auth::login))
}

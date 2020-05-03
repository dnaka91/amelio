//! All routes for the server.

use rocket::get;

use crate::templates;

#[get("/")]
pub const fn index() -> templates::Index {
    templates::Index
}
//! All routes for the server.

#![allow(clippy::missing_const_for_fn)]
#![cfg_attr(doc, allow(unused_braces))]

use rocket::response::Redirect;
use rocket::{get, uri};

use crate::roles::AuthUser;
use crate::templates;

pub mod assets;
pub mod auth;
pub mod courses;
pub mod errors;
pub mod users;

/// Index page for authenticated users.
#[get("/")]
pub const fn index_user(user: &AuthUser) -> templates::Index {
    templates::Index { role: user.0.role }
}

/// Index page for non-authenticated users, redirecting to the login page.
#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(auth::login))
}

macro_rules! enum_from_form_value {
    ($t:ty) => {
        impl<'v> rocket::request::FromFormValue<'v> for $t {
            type Error = &'v rocket::http::RawStr;

            fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<Self, Self::Error> {
                if let Ok(v) = form_value.parse::<String>() {
                    v.parse().map_err(|_| form_value)
                } else {
                    Err(form_value)
                }
            }
        }
    };
}

enum_from_form_value!(crate::models::Role);

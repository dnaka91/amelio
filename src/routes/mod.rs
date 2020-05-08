//! All routes for the server.

#![cfg_attr(doc, allow(unused_braces))]

use rocket::response::Redirect;
use rocket::{get, uri};

use crate::roles::AuthUser;
use crate::templates;

pub mod assets;
pub mod auth;
pub mod users;

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

macro_rules! enum_from_form_value {
    ($t:ty) => {
        impl<'v> rocket::request::FromFormValue<'v> for $t {
            type Error = &'v rocket::http::RawStr;

            fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<Self, Self::Error> {
                match form_value.parse::<String>() {
                    Ok(v) => match v.parse() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(form_value),
                    },
                    Err(_) => Err(form_value),
                }
            }
        }
    };
}

enum_from_form_value!(crate::models::Role);

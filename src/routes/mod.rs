//! All routes for the server.

#![allow(clippy::missing_const_for_fn)]
#![cfg_attr(doc, allow(unused_braces))]

use std::convert::{TryFrom, TryInto};

use log::error;
use rocket::http::{RawStr, Status};
use rocket::request::{FromFormValue, FromParam};
use rocket::response::{self, Redirect, Responder};
use rocket::{get, uri, Request};

use crate::models::Id;
use crate::roles::AuthUser;
use crate::templates;

pub mod assets;
pub mod auth;
pub mod courses;
pub mod errors;
pub mod fairing;
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

/// A wrapper around [`anyhow::Error`] that will print the error and respond with a
/// [`Status::InternalServerError`].
#[derive(Debug)]
pub struct ServerError(anyhow::Error);

impl From<anyhow::Error> for ServerError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl<'r> Responder<'r> for ServerError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        error!("{:?}", self.0);
        Err(Status::InternalServerError)
    }
}

macro_rules! enum_from_form_value {
    ($t:ty) => {
        impl<'v> FromFormValue<'v> for $t {
            type Error = &'v RawStr;

            fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
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

macro_rules! from_request {
    ($t:ty) => {
        impl<'v> FromFormValue<'v> for $t {
            type Error = &'v RawStr;

            fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
                form_value.try_into()
            }
        }

        impl<'a> FromParam<'a> for $t {
            type Error = &'a RawStr;

            fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
                param.try_into()
            }
        }
    };
}

/// A string that is guaranteed to not be empty when parsed from a request param or form value.
pub struct NonEmptyString(String);

impl<'a> TryFrom<&'a RawStr> for NonEmptyString {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value.parse::<String>().map_err(|_| value)?;

        if parsed.is_empty() {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

from_request!(NonEmptyString);

/// An ID that is guaranteed to equal or greater than `1` when parsed from a request param or form
/// value.
pub struct PositiveId(Id);

impl<'a> TryFrom<&'a RawStr> for PositiveId {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value.parse::<Id>().map_err(|_| value)?;

        if parsed <= 0 {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

from_request!(PositiveId);

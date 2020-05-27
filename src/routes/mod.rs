//! All routes for the server.

#![allow(clippy::missing_const_for_fn)]
#![cfg_attr(doc, allow(unused_braces))]

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use log::error;
use num_traits::PrimInt;
use rocket::http::{RawStr, Status};
use rocket::request::{FromFormValue, FromParam};
use rocket::response::{self, Redirect, Responder};
use rocket::{get, uri, Request, UriDisplayPath};
use url::Url;

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::models::Id;
use crate::roles::AuthUser;
use crate::services::{self, TicketService};
use crate::templates;

pub mod assets;
pub mod auth;
pub mod courses;
pub mod errors;
pub mod fairing;
pub mod tickets;
pub mod users;

/// Index page for authenticated users.
#[get("/")]
pub fn index_user(user: &AuthUser, conn: DbConn) -> Result<templates::Index, ServerError> {
    let service = services::ticket_service(
        repositories::ticket_repo(&conn),
        repositories::course_repo(&conn),
    );

    let created_tickets = service.list_created(user.0.id)?;
    let assigned_tickets = service.list_assigned(user.0.id, user.0.role)?;

    Ok(templates::Index {
        role: user.0.role,
        name: user.0.name.clone(),
        created_tickets,
        assigned_tickets,
    })
}

/// Index page for non-authenticated users, redirecting to the login page.
#[get("/", rank = 2)]
pub fn index() -> Redirect {
    Redirect::to(uri!(auth::login))
}

/// FAQ page for authenticated users.
#[get("/faq")]
pub fn faq_user(user: &AuthUser) -> templates::Faq {
    templates::Faq { role: user.0.role }
}

/// FAQ page for non-authenticated users, redirecting to the login page.
#[get("/faq", rank = 2)]
pub fn faq() -> Redirect {
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

macro_rules! enum_from_request {
    ($t:ty) => {
        impl<'a> TryFrom<&'a RawStr> for $t {
            type Error = &'a RawStr;

            fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
                if let Ok(v) = value.url_decode() {
                    v.parse().map_err(|_| value)
                } else {
                    Err(value)
                }
            }
        }

        from_request!($t);
    };
}

enum_from_request!(crate::models::Role);
enum_from_request!(crate::models::TicketType);
enum_from_request!(crate::models::Category);
enum_from_request!(crate::models::Priority);
enum_from_request!(crate::models::Status);

/// A string that is guaranteed to not be empty when parsed from a request param or form value.
pub struct NonEmptyString(String);

impl<'a> TryFrom<&'a RawStr> for NonEmptyString {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value.url_decode().map_err(|_| value)?;

        if parsed.is_empty() {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

from_request!(NonEmptyString);

/// An ID that is guaranteed to be equal or greater than `1` when parsed from a request param or
/// form value.
pub type PositiveId = PositiveNum<Id>;

/// An integer that is guaranteed to equal or greater than `1` when parsed from a request param or form
/// value.
#[derive(UriDisplayPath)]
pub struct PositiveNum<N: PrimInt>(N);

impl<'a, N: PrimInt + FromStr> TryFrom<&'a RawStr> for PositiveNum<N> {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value
            .url_decode()
            .map_err(|_| value)
            .and_then(|v| v.parse().map_err(|_| value))?;

        if parsed <= N::zero() {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

impl<'v, N: PrimInt + FromStr> FromFormValue<'v> for PositiveNum<N> {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        form_value.try_into()
    }
}

impl<'a, N: PrimInt + FromStr> FromParam<'a> for PositiveNum<N> {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
        param.try_into()
    }
}

/// An integer that represents an hour in the range of `0-23`.
pub struct Hour(u8);

impl<'a> TryFrom<&'a RawStr> for Hour {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value
            .url_decode()
            .map_err(|_| value)
            .and_then(|v| v.parse().map_err(|_| value))?;

        if parsed >= 24 {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

from_request!(Hour);

/// An integer that represents a minute in the range of `0-59`.
pub struct Minute(u8);

impl<'a> TryFrom<&'a RawStr> for Minute {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value
            .url_decode()
            .map_err(|_| value)
            .and_then(|v| v.parse().map_err(|_| value))?;

        if parsed >= 60 {
            Err(value)
        } else {
            Ok(Self(parsed))
        }
    }
}

from_request!(Minute);

/// An integer that represents a second in the range of `0-59`.
type Second = Minute;

/// An [`Url`] that's guaranteed to be valid.
pub struct ValidUrl(Url);

impl<'a> TryFrom<&'a RawStr> for ValidUrl {
    type Error = &'a RawStr;

    fn try_from(value: &'a RawStr) -> Result<Self, Self::Error> {
        let parsed = value
            .url_decode()
            .map_err(|_| value)
            .and_then(|v| v.parse().map_err(|_| value))?;

        Ok(Self(parsed))
    }
}

from_request!(ValidUrl);

//! User authentication and role management related functionality.

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

use crate::db::connection::DbConn;
use crate::db::repositories::{self, UserRepository};
use crate::models::{Id, Role, User};

/// Any user that is authenticated but not checked to have a specific role.
pub struct AuthUser(User);

impl<'a, 'r> FromRequest<'a, 'r> for &'a AuthUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user_result = request.local_cache(|| {
            let conn = request.guard::<DbConn>().succeeded()?;
            let repo = repositories::user_repo(&conn);

            request
                .cookies()
                .get_private("session")
                .and_then(|cookie| cookie.value().parse::<Id>().ok())
                .and_then(|id| repo.find(id).ok())
                .map(AuthUser)
        });

        user_result.as_ref().or_forward(())
    }
}

/// The admin user has the [`Role::Admin`] role, has access to all features and especially can
/// manage user accounts.
pub struct AdminUser<'a>(pub &'a User);

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = request.guard::<&AuthUser>()?;

        if user.0.role == Role::Admin {
            Outcome::Success(AdminUser(&user.0))
        } else {
            Outcome::Forward(())
        }
    }
}

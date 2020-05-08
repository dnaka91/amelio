//! User authentication and role management related functionality.

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::Request;

use crate::db::connection::DbConn;
use crate::db::repositories::{self, UserRepository};
use crate::models::User;

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
                .and_then(|cookie| cookie.value().parse::<i32>().ok())
                .and_then(|id| repo.find(id).ok())
                .map(AuthUser)
        });

        user_result.as_ref().or_forward(())
    }
}

//! User authentication and role management related functionality.

use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

use crate::db::connection::DbConn;
use crate::db::repositories::{self, UserRepository};
use crate::models::{Id, Role, User};

/// Any user that is authenticated but not checked to have a specific role.
pub struct AuthUser(pub User);

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

/// The opposite of an [`AuthUser`], a user that is **not** authenticated.
pub struct NoUser;

impl<'a, 'r> FromRequest<'a, 'r> for NoUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if request.guard::<&AuthUser>().is_forward() {
            Outcome::Success(Self)
        } else {
            Outcome::Forward(())
        }
    }
}

macro_rules! ranked_user_guard {
    (
        $(#[$docs:meta])*
        $name:ident, $role:expr
    ) => {
        $(#[$docs])*
        pub struct $name<'a>(pub &'a User);

        impl<'a, 'r> FromRequest<'a, 'r> for $name<'a> {
            type Error = ();

            fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
                let user = request.guard::<&AuthUser>()?;

                if user.0.role <= $role {
                    Outcome::Success(Self(&user.0))
                } else {
                    Outcome::Forward(())
                }
            }
        }
    };
}

ranked_user_guard!(
    /// The admin user has the [`Role::Admin`] role, has access to all features and especially can
    /// manage user accounts.
    AdminUser,
    Role::Admin
);

ranked_user_guard!(
    /// The author user has the [`Role::Author`] role and has the same capabilities as the tutor.
    /// This user's main role is to solve tickets the tutor couldn't and update media based on the
    /// request.
    AuthorUser,
    Role::Author
);

ranked_user_guard!(
    /// The tutor user has the [`Role::Tutor`] role and has access to incoming tickets. This user
    /// can accept, reject or close reported tickets. In addition he can forward assigned tickets to
    /// the author.
    TutorUser,
    Role::Tutor
);

ranked_user_guard!(
    /// The student user has the [`Role::Student`] role and has the least amount of rights. A
    /// student can create and search for tickets only.
    StudentUser,
    Role::Student
);

#[cfg(test)]
mod tests {
    use crate::models::Role;

    /// Make sure our role order works as expected.
    #[test]
    fn role_order() {
        assert!(Role::Admin < Role::Author);
        assert!(Role::Author < Role::Tutor);
        assert!(Role::Tutor < Role::Student);
    }
}

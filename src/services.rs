//! Services of the application which contain the business logic.

use anyhow::{ensure, Result};

use crate::db::repositories::UserRepository;

/// The login service manages the user login. Logout is directly handled in the
/// [`post_logout`](crate::routes::auth::post_logout) route because that logic is part of the
/// framework.
pub trait LoginService {
    /// Try to login a user and return its database ID if successful.
    fn login(&self, cred: &Credentials) -> Result<i32>;
}

/// The credentials that a user needs to authenticate.
pub struct Credentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

/// Main implementation of [`LoginService`].
struct LoginServiceImpl<R: UserRepository> {
    user_repo: R,
}

impl<R: UserRepository> LoginService for LoginServiceImpl<R> {
    fn login(&self, cred: &Credentials) -> Result<i32> {
        self.user_repo
            .find_by_username(cred.username)
            .and_then(|user| {
                ensure!(
                    user.password == cred.password,
                    "Invalid username or password"
                );
                Ok(user.id)
            })
    }
}

/// Create a new login service.
pub fn login_service(user_repo: impl UserRepository) -> impl LoginService {
    LoginServiceImpl { user_repo }
}

//! Services of the application which contain the business logic.

use std::iter;

use anyhow::{ensure, Result};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::db::repositories::UserRepository;
use crate::email::{Mail, MailRenderer, MailSender};
use crate::models::{Id, NewUser, Role, User};

/// The login service manages the user login. Logout is directly handled in the
/// [`post_logout`](crate::routes::auth::post_logout) route because that logic is part of the
/// framework.
pub trait LoginService {
    /// Try to login a user and return its database ID if successful.
    fn login(&self, cred: &Credentials) -> Result<Id>;
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
    fn login(&self, cred: &Credentials) -> Result<Id> {
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

/// The user service manages users of the system, mainly creation and activation and deactivation.
pub trait UserService {
    /// List all active and inactive users.
    fn list(&self) -> Result<(Vec<User>, Vec<User>)>;
    /// Create a new user in the system.
    fn create(&self, username: String, name: String, role: Role) -> Result<()>;
    /// Activate a previously created user.
    fn activate(&self, code: &str, password: &str) -> Result<()>;
    /// Enable or disable a user.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
}

/// Main implementation of [`UserRepository`].
struct UserServiceImpl<R: UserRepository, MS: MailSender, MR: MailRenderer> {
    user_repo: R,
    mail_sender: MS,
    mail_renderer: MR,
}

impl<R, MS, MR> UserServiceImpl<R, MS, MR>
where
    R: UserRepository,
    MS: MailSender,
    MR: MailRenderer,
{
    /// Generate a new code for activating new user accounts.
    fn generate_code() -> String {
        let mut rng = rand::thread_rng();

        iter::repeat(())
            .map(|_| rng.sample(Alphanumeric))
            .take(20)
            .collect()
    }
}

impl<R, MS, MR> UserService for UserServiceImpl<R, MS, MR>
where
    R: UserRepository,
    MS: MailSender,
    MR: MailRenderer,
{
    fn list(&self) -> Result<(Vec<User>, Vec<User>)> {
        self.user_repo.list().map(|users| {
            users
                .into_iter()
                .filter(|u| u.code.is_empty())
                .partition(|u| u.active)
        })
    }

    fn create(&self, username: String, name: String, role: Role) -> Result<()> {
        let code = Self::generate_code();
        self.user_repo.create(NewUser {
            username: username.clone(),
            name: name.clone(),
            role,
            code: code.clone(),
        })?;

        let (subject, message) = self.mail_renderer.invitation(&name, &code);

        self.mail_sender.send(Mail {
            from: ("amelio@dnaka91.rocks", "Amelio"),
            to: (&format!("{}@iubh-fernstudium.de", username), &name),
            subject,
            message: &message,
        })?;

        Ok(())
    }

    fn activate(&self, code: &str, password: &str) -> Result<()> {
        let resp = self.user_repo.activate(code, password)?;

        ensure!(resp == 1, "Activation code is invalid");
        Ok(())
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        self.user_repo.enable(id, enable).map_err(Into::into)
    }
}

/// Create a new user service.
pub fn user_service(
    user_repo: impl UserRepository,
    mail_sender: impl MailSender,
    mail_renderer: impl MailRenderer,
) -> impl UserService {
    UserServiceImpl {
        user_repo,
        mail_sender,
        mail_renderer,
    }
}

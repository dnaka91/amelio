//! Services of the application which contain the business logic.

use std::iter;

use anyhow::{ensure, Result};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::db::repositories::{CourseRepository, UserRepository};
use crate::email::{Mail, MailRenderer, MailSender};
use crate::hashing::Hasher;
use crate::models::{CourseWithNames, Id, NewCourse, NewUser, Role, User};

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
struct LoginServiceImpl<R: UserRepository, H: Hasher> {
    user_repo: R,
    hasher: H,
}

impl<R: UserRepository, H: Hasher> LoginService for LoginServiceImpl<R, H> {
    fn login(&self, cred: &Credentials) -> Result<Id> {
        self.user_repo
            .find_by_username(cred.username)
            .and_then(|user| {
                ensure!(
                    self.hasher.verify(cred.password, &user.password)?,
                    "Invalid username or password"
                );
                Ok(user.id)
            })
    }
}

/// Create a new login service.
pub fn login_service(user_repo: impl UserRepository, hasher: impl Hasher) -> impl LoginService {
    LoginServiceImpl { user_repo, hasher }
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
struct UserServiceImpl<R: UserRepository, MS: MailSender, MR: MailRenderer, H: Hasher> {
    user_repo: R,
    mail_sender: MS,
    mail_renderer: MR,
    hasher: H,
}

impl<R, MS, MR, H> UserServiceImpl<R, MS, MR, H>
where
    R: UserRepository,
    MS: MailSender,
    MR: MailRenderer,
    H: Hasher,
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

impl<R, MS, MR, H> UserService for UserServiceImpl<R, MS, MR, H>
where
    R: UserRepository,
    MS: MailSender,
    MR: MailRenderer,
    H: Hasher,
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
        let hash = self.hasher.hash(password)?;

        self.user_repo.activate(code, &hash)
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
    hasher: impl Hasher,
) -> impl UserService {
    UserServiceImpl {
        user_repo,
        mail_sender,
        mail_renderer,
        hasher,
    }
}

/// A list of authors and tutors with only their ID and name.
///
/// The first tuple element contains the authors and the second one the tutors.
type VecAuthorsTutors = (Vec<(i32, String)>, Vec<(i32, String)>);

/// The course service manages courses of the system, like listing existing ones, enable or disable
/// them or adding new ones.
pub trait CourseService {
    /// List all courses together with their author and tutor names.
    fn list(&self) -> Result<Vec<CourseWithNames>>;
    /// List all authors and tutors with ID and name.
    fn list_authors_tutors(&self) -> Result<VecAuthorsTutors>;
    /// Create a new course in the system.
    fn create(&self, code: String, title: String, author_id: Id, tutor_id: Id) -> Result<()>;
    /// Enable or disable a course.
    fn enable(&self, id: i32, enable: bool) -> Result<()>;
}

/// Main implementation of [`CourseService`].
struct CourseServiceImpl<UR: UserRepository, CR: CourseRepository> {
    user_repo: UR,
    course_repo: CR,
}

impl<UR: UserRepository, CR: CourseRepository> CourseService for CourseServiceImpl<UR, CR> {
    fn list(&self) -> Result<Vec<CourseWithNames>> {
        self.course_repo.list_with_names()
    }

    fn list_authors_tutors(&self) -> Result<VecAuthorsTutors> {
        Ok((
            self.user_repo.list_names_by_role(Role::Author)?,
            self.user_repo.list_names_by_role(Role::Tutor)?,
        ))
    }

    fn create(&self, code: String, title: String, author_id: Id, tutor_id: Id) -> Result<()> {
        self.course_repo.create(NewCourse {
            code,
            title,
            author_id,
            tutor_id,
        })
    }

    fn enable(&self, id: i32, enable: bool) -> Result<()> {
        self.course_repo.enable(id, enable).map_err(Into::into)
    }
}

pub fn course_service(
    user_repo: impl UserRepository,
    course_repo: impl CourseRepository,
) -> impl CourseService {
    CourseServiceImpl {
        user_repo,
        course_repo,
    }
}

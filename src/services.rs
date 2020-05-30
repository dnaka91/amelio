//! Services of the application which contain the business logic.

use std::iter;

use anyhow::{ensure, Result};
use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::db::repositories::{CourseRepository, TicketRepository, UserRepository};
use crate::email::{Mail, MailRenderer, MailSender};
use crate::hashing::Hasher;
use crate::models::{
    Category, Course, CourseWithNames, EditCourse, EditTicket, EditUser, Id, NewComment, NewCourse,
    NewMedium, NewTicket, NewUser, Priority, Role, Status, TicketSearch, TicketWithNames,
    TicketWithRels, User,
};

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
    /// Get a single user by its ID.
    fn get(&self, id: Id) -> Result<User>;
    /// Activate a previously created user.
    fn activate(&self, code: &str, password: &str) -> Result<()>;
    /// Enable or disable a user.
    fn enable(&self, id: Id, enable: bool) -> Result<()>;
    /// Update the details of a user.
    fn update(&self, id: Id, name: String, role: Role) -> Result<()>;
}

/// Main implementation of [`UserService`].
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

    fn get(&self, id: Id) -> Result<User> {
        self.user_repo.find(id)
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

    fn update(&self, id: Id, name: String, role: Role) -> Result<()> {
        self.user_repo.update(EditUser { id, name, role })
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
    /// Get a single course by its ID.
    fn get(&self, id: Id) -> Result<Course>;
    /// Create a new course in the system.
    fn create(&self, code: String, title: String, author_id: Id, tutor_id: Id) -> Result<()>;
    /// Enable or disable a course.
    fn enable(&self, id: Id, enable: bool) -> Result<()>;
    /// Update the information of a course.
    fn update(&self, id: Id, title: String, author_id: Id, tutor_id: Id) -> Result<()>;
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

    fn get(&self, id: Id) -> Result<Course> {
        self.course_repo.get(id)
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

    fn update(&self, id: Id, title: String, author_id: Id, tutor_id: Id) -> Result<()> {
        self.course_repo.update(EditCourse {
            id,
            title,
            author_id,
            tutor_id,
        })
    }
}

/// Create a new course service.
pub fn course_service(
    user_repo: impl UserRepository,
    course_repo: impl CourseRepository,
) -> impl CourseService {
    CourseServiceImpl {
        user_repo,
        course_repo,
    }
}

/// The ticket service manages tickets of the system, like listing existing or adding new ones.
pub trait TicketService {
    /// List all tickets.
    fn list(&self) -> Result<Vec<TicketWithNames>>;
    /// List all tickets that were created by the given user.
    fn list_created(&self, user_id: Id) -> Result<Vec<TicketWithNames>>;
    /// List all tickets that are currently assigned to the given user.
    fn list_assigned(&self, user_id: Id, role: Role) -> Result<Vec<TicketWithNames>>;
    /// List all courses with ID and name.
    fn list_course_names(&self) -> Result<Vec<(Id, String)>>;
    /// Get a single ticket by its ID.
    fn get(&self, id: Id) -> Result<TicketWithNames>;
    /// Get a single ticket together with all relations. If the opening user is a tutor or author
    /// and the ticket is still in [`Status::Open`] it will be changed to [`Status::InProgress`].
    fn get_with_rels(&self, id: Id, user_id: Id, role: Role) -> Result<TicketWithRels>;
    /// Create a new ticket in the system.
    fn create(&self, ticket: NewTicket, medium: NewMedium) -> Result<Id>;
    /// Add a new comment to a ticket.
    fn add_comment(&self, id: Id, creator_id: Id, message: String) -> Result<()>;
    /// Update the details of a ticket.
    fn update(&self, id: Id, priority: Priority) -> Result<()>;
    /// Forward a ticket to its course's author.
    fn forward(&self, id: Id) -> Result<()>;
    /// Change the current status of the ticket.
    fn change_status(&self, id: Id, status: Status) -> Result<()>;
    /// Search for tickets with different criteria.
    fn search(&self, role: Role, search: &mut TicketSearch) -> Result<Vec<TicketWithNames>>;
    /// Check whether the user can open a specific ticket.
    fn can_open(&self, id: Id, user_id: Id, role: Role) -> Result<bool>;
}

/// Main implementation of [`TicketService`].
struct TicketServiceImpl<TR: TicketRepository, CR: CourseRepository> {
    ticket_repo: TR,
    course_repo: CR,
}

impl<TR: TicketRepository, CR: CourseRepository> TicketServiceImpl<TR, CR> {
    /// Decide the priority of a ticket based on its category.
    fn map_priority(category: Category) -> Priority {
        match category {
            Category::Editorial => Priority::Medium,
            Category::Content => Priority::High,
            Category::Improvement | Category::Addition => Priority::Low,
        }
    }
}

impl<TR: TicketRepository, CR: CourseRepository> TicketService for TicketServiceImpl<TR, CR> {
    fn list(&self) -> Result<Vec<TicketWithNames>> {
        self.ticket_repo.list_with_names()
    }

    fn list_created(&self, user_id: Id) -> Result<Vec<TicketWithNames>> {
        self.ticket_repo.list_by_creator_id(user_id)
    }

    fn list_assigned(&self, user_id: Id, role: Role) -> Result<Vec<TicketWithNames>> {
        if role <= Role::Tutor {
            self.ticket_repo.list_by_assignee_id(user_id)
        } else {
            Ok(Vec::new())
        }
    }

    fn list_course_names(&self) -> Result<Vec<(Id, String)>> {
        self.course_repo.list_names()
    }

    fn get(&self, id: Id) -> Result<TicketWithNames> {
        self.ticket_repo.get_with_names(id)
    }

    fn get_with_rels(&self, id: Id, user_id: Id, role: Role) -> Result<TicketWithRels> {
        // If we open a ticket as tutor or author, update the status first.
        if role == Role::Tutor || role == Role::Author {
            self.ticket_repo.activate_ticket(id, user_id)?;
        }

        self.ticket_repo.get_with_rels(id)
    }

    fn create(&self, ticket: NewTicket, medium: NewMedium) -> Result<Id> {
        let priority = Self::map_priority(ticket.category);

        self.ticket_repo.create(ticket, priority, medium)
    }

    fn add_comment(&self, id: Id, creator_id: Id, message: String) -> Result<()> {
        self.ticket_repo.add_comment(NewComment {
            ticket_id: id,
            creator_id,
            timestamp: Utc::now(),
            message,
        })
    }

    fn update(&self, id: Id, priority: Priority) -> Result<()> {
        self.ticket_repo.update(EditTicket { id, priority })
    }

    fn forward(&self, id: Id) -> Result<()> {
        self.ticket_repo.forward(id)
    }

    fn change_status(&self, id: Id, status: Status) -> Result<()> {
        let current_status = self.ticket_repo.get_status(id)?;
        ensure!(
            current_status.can_change(status),
            "Status cannot be changed"
        );

        self.ticket_repo.set_status(id, status)
    }

    fn search(&self, role: Role, mut search: &mut TicketSearch) -> Result<Vec<TicketWithNames>> {
        if role >= Role::Student {
            search.priority = None;
        }

        self.ticket_repo.search(search)
    }

    fn can_open(&self, id: Id, user_id: Id, role: Role) -> Result<bool> {
        // Everyone above a student can always see any ticket details
        if role < Role::Student {
            return Ok(true);
        }

        self.ticket_repo.is_creator(id, user_id)
    }
}

/// Create a new ticket service.
pub fn ticket_service(
    ticket_repo: impl TicketRepository,
    course_repo: impl CourseRepository,
) -> impl TicketService {
    TicketServiceImpl {
        ticket_repo,
        course_repo,
    }
}

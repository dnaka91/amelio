//! All templates that are used to render the web pages of this service.

use askama::Template;
use chrono::Timelike;
use strum::{AsRefStr, EnumString};

use crate::language::Translate;
use crate::models::{
    Category, Course, CourseWithNames, Id, Medium, MediumType, Priority, Role, Status,
    TicketSearch, TicketType, TicketWithNames, TicketWithRels, User,
};

mod filters {
    //! Custom filters for [`askama`] templates.
    #![allow(clippy::unnecessary_wraps)]

    use chrono::prelude::*;
    use chrono_tz::Europe::Berlin;

    /// Convert an image URL into a source set with different DPI scaling and output the `src` and
    /// `srcset` attributes for an `<img>` element.
    ///
    /// The different images are expected to be located next to the original image and to have a
    /// suffix in the form `@<scaling>x`. For example, an input of `logo.png` will create an output
    /// as follows:
    ///
    /// ```text
    /// src="logo.png" srcset="logo.png, logo@1.5x.png 1.5x, logo@2x.png 2x, ..."
    /// ```
    pub fn srcset(base: &str) -> askama::Result<String> {
        Ok(base.rfind('.').map_or_else(
            || base.to_owned(),
            |pos| {
                format!(
                    "src=\"{0}\" srcset=\"{0}, \
                    {name}@1.5x{ext} 1.5x, \
                    {name}@2x{ext} 2x, \
                    {name}@3x{ext} 3x, \
                    {name}@4x{ext} 4x\"",
                    base,
                    name = &base[..pos],
                    ext = &base[pos..],
                )
            },
        ))
    }

    /// Convert a UTC timestamp into German time zone and then print it in a custom format fitting
    /// for comments.
    pub fn timestamp(ts: &DateTime<Utc>) -> askama::Result<String> {
        Ok(ts
            .with_timezone(&Berlin)
            .format("um %H:%M am %d.%m.%Y")
            .to_string())
    }

    /// Return the string representation of a value if it exists or an empty string if it's
    /// [`None`].
    pub fn opt_str<T: AsRef<str>>(opt: &Option<T>) -> askama::Result<&str> {
        Ok(opt.as_ref().map_or("", AsRef::as_ref))
    }

    /// Compare two values and return ` selected` if they match, an empty string otherwise. This is
    /// helpful in pre-selecting a value in HTML `<select>` elements.
    pub fn select<T: Eq>(value: &T, other: &T) -> askama::Result<&'static str> {
        Ok(if value == other { " selected" } else { "" })
    }

    /// Compare two values exactly as [`select`] but with the first value being optional.
    /// If the first value is [`None`], an empty string is returned.
    pub fn opt_select<T: Eq>(opt: &Option<T>, other: &T) -> askama::Result<&'static str> {
        opt.as_ref().map_or(Ok(""), |value| select(value, other))
    }
}

/// The color trait allows to tie a specific color to the object that implements it.
///
/// This is mostly useful for enums that should be shown in different colors within the template.
/// The color values are CSS classes and bound to the used framework.
trait Color {
    /// The color to be used within a tag element.
    fn tag(&self) -> &'static str;
}

impl Color for Status {
    fn tag(&self) -> &'static str {
        match self {
            Self::Open => "is-primary",
            Self::InProgress => "is-info",
            Self::Accepted => "is-success",
            Self::Refused => "is-danger",
            Self::Completed => "is-light",
        }
    }
}

/// The icon trait allows to show an icon representation of the implementing object within a
/// template.
///
/// This is, similar to the [`Color`] trait, mostly useful for enums that should be shown with
/// different icons. The values are CSS classes and bound tot the used icon font.
trait Icon {
    /// The icon to be shown.
    fn icon(&self) -> &'static str;
}

impl Icon for Status {
    fn icon(&self) -> &'static str {
        match self {
            Self::Open => "fa-envelope",
            Self::InProgress => "fa-cogs",
            Self::Accepted => "fa-check",
            Self::Refused => "fa-times",
            Self::Completed => "fa-archive",
        }
    }
}

/// Different message codes that can be send as flash messages and translated to different
/// languages.
#[derive(Copy, Clone, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum MessageCode {
    // Error codes
    InvalidCredentials,
    FailedUserCreation,
    FailedUserUpdate,
    InvalidCodeOrError,
    FailedCourseCreation,
    FailedCourseUpdate,
    FailedTicketCreation,
    FailedTicketUpdate,
    FailedCommentCreation,
    // Success codes
    UserCreated,
    UserUpdated,
    UserActivated,
    CourseCreated,
    CourseUpdated,
    TicketCreated,
    TicketUpdated,
    CommentCreated,
    // Unknown
    Unknown,
}

impl From<&str> for MessageCode {
    #[inline]
    fn from(value: &str) -> Self {
        if let Ok(v) = value.parse() {
            v
        } else {
            Self::Unknown
        }
    }
}

impl Translate for MessageCode {
    fn german(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "Ung\u{00fc}ltiger Nutzername oder Passwort",
            Self::FailedUserCreation => "Benutzererstellung fehlgeschlagen",
            Self::FailedUserUpdate => "Benutzerbearbeitung fehlgeschlagen",
            Self::InvalidCodeOrError => " Ung\u{00fc}ltiger Aktivierungscode oder anderer Fehler",
            Self::FailedCourseCreation => "Kurserstellung fehlgeschlagen",
            Self::FailedCourseUpdate => "Kursbearbeitung fehlgeschlagen",
            Self::FailedTicketCreation => "Ticketerstellung fehlgeschlagen",
            Self::FailedTicketUpdate => "Ticketbearbeitung fehlgeschlagen",
            Self::FailedCommentCreation => "Kommentarerstellung fehlgeschlagen",
            Self::UserCreated => "Account erfolgreich erstellt",
            Self::UserUpdated => "Account erfolgreich bearbeitet",
            Self::UserActivated => "Account erfolgreich aktiviert",
            Self::CourseCreated => "Kurs erfolgreich erstellt",
            Self::CourseUpdated => "Kurs erfolgreich bearbeitet",
            Self::TicketCreated => "Ticket erfolgreich erstellt",
            Self::TicketUpdated => "Ticket erfolgreich bearbeitet",
            Self::CommentCreated => "Kommentar erfolgreich erstellt",
            Self::Unknown => "Unbekannter Fehler",
        }
    }
}

/// Template for the index page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub role: Role,
    pub name: String,
    pub created_tickets: Vec<TicketWithNames>,
    pub assigned_tickets: Vec<TicketWithNames>,
}

/// Template for the login page.
#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    /// Optional flash message that's shown as an error.
    pub flash: Option<(String, MessageCode)>,
}

/// Template for the FAQ page.
#[derive(Template)]
#[template(path = "faq.html")]
pub struct Faq {
    pub role: Role,
}

/// Template for the user list page.
#[derive(Template)]
#[template(path = "users/index.html")]
pub struct Users {
    pub role: Role,
    pub flash: Option<(String, MessageCode)>,
    pub active: Vec<User>,
    pub inactive: Vec<User>,
}

/// Template for the new user page.
#[derive(Template)]
#[template(path = "users/new.html")]
pub struct NewUser {
    pub role: Role,
    pub flash: Option<MessageCode>,
}

/// Template for the user activation page.
#[derive(Template)]
#[template(path = "users/activate.html")]
pub struct ActivateUser {
    pub flash: Option<MessageCode>,
    pub code: String,
}

/// Template for the edit user page.
#[derive(Template)]
#[template(path = "users/edit.html")]
pub struct EditUser {
    pub role: Role,
    pub flash: Option<MessageCode>,
    pub user: User,
}

/// Template for the user list page.
#[derive(Template)]
#[template(path = "courses/index.html")]
pub struct Courses {
    pub role: Role,
    pub flash: Option<(String, MessageCode)>,
    pub courses: Vec<CourseWithNames>,
}

/// Template for the new course page.
#[derive(Template)]
#[template(path = "courses/new.html")]
pub struct NewCourse {
    pub role: Role,
    pub flash: Option<MessageCode>,
    pub authors: Vec<(Id, String)>,
    pub tutors: Vec<(Id, String)>,
}

/// Template for the edit course page.
#[derive(Template)]
#[template(path = "courses/edit.html")]
pub struct EditCourse {
    pub role: Role,
    pub flash: Option<MessageCode>,
    pub authors: Vec<(Id, String)>,
    pub tutors: Vec<(Id, String)>,
    pub course: Course,
}

/// Template for the new ticket page.
#[derive(Template)]
#[template(path = "tickets/new/index.html")]
pub struct NewTicket {
    pub role: Role,
    pub ty: TicketType,
    pub courses: Vec<(Id, String)>,
}

/// Template for the ticket detail page.
#[derive(Template)]
#[template(path = "tickets/edit/index.html")]
pub struct TicketDetail {
    pub role: Role,
    pub flash: Option<(String, MessageCode)>,
    pub ticket: TicketWithRels,
}

/// Template for the ticket search page.
#[derive(Template)]
#[template(path = "tickets/search.html")]
pub struct SearchTickets {
    pub role: Role,
    pub user_id: Id,
    pub tickets: Vec<TicketWithNames>,
    pub courses: Vec<(Id, String)>,
    pub search: TicketSearch,
}

/// Template for the _403 Forbidden_ error.
#[derive(Template)]
#[template(path = "errors/403.html")]
pub struct Error403;

/// Template for the _404 Not Found_ error.
#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct Error404;

/// Template for the _500 Internal Server Error_ error.
#[derive(Template)]
#[template(path = "errors/500.html")]
pub struct Error500;

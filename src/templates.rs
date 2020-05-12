//! All templates that are used to render the web pages of this service.

use askama::Template;
use strum::{AsRefStr, EnumString};

use crate::models::{CourseWithNames, Role, User};

mod filters {
    //! Custom filters for [`askama`] templates.

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
        Ok(if let Some(pos) = base.rfind('.') {
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
        } else {
            base.to_owned()
        })
    }
}

/// The translate trait allows for any implementing object to translate itself or its value into
/// different languages.
trait Translate {
    /// Translate to German language.
    fn german(&self) -> &'static str;
}

impl Translate for Role {
    fn german(&self) -> &'static str {
        match self {
            Self::Admin => "Administrator",
            Self::Author => "Autor",
            Self::Tutor => "Tutor",
            Self::Student => "Student",
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
    InvalidCodeOrError,
    // Success codes
    UserActivated,
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
            Self::InvalidCodeOrError => " Ung\u{00fc}ltiger Aktivierungscode oder anderer Fehler",
            Self::UserActivated => "Account erfolgreich aktiviert",
            Self::Unknown => "Unbekannter Fehler",
        }
    }
}

/// Template for the index page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

/// Template for the login page.
#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    /// Optional flash message that's shown as an error.
    pub flash: Option<(String, MessageCode)>,
}

/// Template for the user list page.
#[derive(Template)]
#[template(path = "users/index.html")]
pub struct Users {
    pub active: Vec<User>,
    pub inactive: Vec<User>,
}

/// Template for the new user page.
#[derive(Template)]
#[template(path = "users/new.html")]
pub struct NewUser {
    pub flash: Option<MessageCode>,
}

/// Template for the user activation page.
#[derive(Template)]
#[template(path = "users/activate.html")]
pub struct ActivateUser {
    pub flash: Option<MessageCode>,
    pub code: String,
}

/// Template for the user list page.
#[derive(Template)]
#[template(path = "courses/index.html")]
pub struct Courses {
    pub courses: Vec<CourseWithNames>,
}

/// Template for the _403 Forbidden_ error.
#[derive(Template)]
#[template(path = "errors/403.html")]
pub struct Error403;

/// Template for the _404 Not Found_ error.
#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct Error404;

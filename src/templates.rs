//! All templates that are used to render the web pages of this service.

use askama::Template;

/// Template for the index page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

/// Template for the login page.
#[derive(Template)]
#[template(path = "login.html")]
pub struct Login {
    /// Optional flash message that's shown as an error.
    pub flash: Option<String>,
}

//! All templates that are used to render the web pages of this service.

use askama::Template;

/// Template for the index page.
#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

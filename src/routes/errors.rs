//! Catchers for custom error pages.

use rocket::catch;

use crate::templates;

/// 403 Forbidden error page.
#[catch(403)]
pub const fn forbidden() -> templates::Error403 {
    templates::Error403
}

/// 404 Not Found error page.
#[catch(404)]
pub const fn not_found() -> templates::Error404 {
    templates::Error404
}

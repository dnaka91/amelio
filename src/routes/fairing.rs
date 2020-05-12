//! Special routes that are used by custom [`fairings`](crate::fairings).

use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{get, uri};

/// Endpoint that always returns [`Status::Forbidden`]. This is used by the
/// [`Auth`](crate::fairings::Auth) fairing for unauthorized users.
#[get("/forbidden")]
pub const fn forbidden() -> Status {
    Status::Forbidden
}

/// Endpoint that always forwards to the login page. This is used by the
/// [`Auth`](crate::fairings::Auth) fairing for unauthenticated users.
#[get("/to-login")]
pub fn to_login() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

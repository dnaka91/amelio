use anyhow::Result;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{get, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::roles::{AdminUser, AuthUser};
use crate::services::{self, CourseService};
use crate::templates;

/// Course management page for administrators.
#[get("/")]
pub fn courses_admin(_user: AdminUser, conn: DbConn) -> Result<templates::Courses> {
    let service = services::course_service(repositories::course_repo(&conn));
    let courses = service.list()?;

    Ok(templates::Courses { courses })
}

/// Course management is not allowed for non-admin users.
#[get("/", rank = 2)]
pub const fn courses_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}

/// Course management for everyone else, redirecting to the login page.
#[get("/", rank = 3)]
pub fn courses() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

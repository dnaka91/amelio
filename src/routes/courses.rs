use anyhow::Result;
use log::error;
use rocket::http::Status;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::roles::{AdminUser, AuthUser};
use crate::services::{self, CourseService};
use crate::templates::{self, MessageCode};

/// Course management page for administrators.
#[get("/")]
pub fn courses_admin(
    _user: AdminUser,
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::Courses> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    let courses = service.list()?;

    Ok(templates::Courses {
        flash: flash.map(|f| (f.name().to_owned(), f.msg().into())),
        courses,
    })
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

/// Course creation form for administrators.
#[get("/new")]
pub fn new_course_admin(
    _user: AdminUser,
    flash: Option<FlashMessage<'_, '_>>,
    conn: DbConn,
) -> Result<templates::NewCourse> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    let (authors, tutors) = service.list_authors_tutors()?;

    Ok(templates::NewCourse {
        flash: flash.map(|f| f.msg().into()),
        authors,
        tutors,
    })
}

/// Course creation is not allowed for non-admin users.
#[get("/new", rank = 2)]
pub const fn new_course_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}

/// Course creation for everyone else, redirecting to the login page.
#[get("/new", rank = 3)]
pub fn new_course() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

/// Form data from the course creation form.
#[derive(FromForm)]
pub struct NewCourse {
    code: String,
    title: String,
    author: i32,
    tutor: i32,
}

/// New user POST endpoint to handle course creation, only for administrators.
#[post("/new", data = "<data>")]
pub fn post_new_course_admin(
    _user: AdminUser,
    data: Form<NewCourse>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );

    match service.create(data.0.code, data.0.title, data.0.author, data.0.tutor) {
        Ok(()) => Ok(Flash::success(
            Redirect::to(uri!("/courses", courses)),
            MessageCode::CourseCreated,
        )),
        Err(e) => {
            error!("error during course creation: {:?}", e);
            Err(Flash::error(
                Redirect::to(uri!("/courses", new_course)),
                MessageCode::FailedCourseCreation,
            ))
        }
    }
}

/// Course creation endpoints are not accessible for non-admin users.
#[post("/new", rank = 2)]
pub const fn post_new_course_auth(_user: &AuthUser) -> Status {
    Status::Forbidden
}
/// Course creation endpoint for everyone else, redirecting to the login page.
#[post("/new", rank = 3)]
pub fn post_new_course() -> Redirect {
    Redirect::to(uri!(super::auth::login))
}

//! Course management related routes.

use anyhow::Result;
use log::error;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::roles::AdminUser;
use crate::services::{self, CourseService};
use crate::templates::{self, MessageCode};

/// Course management page for administrators.
#[get("/")]
pub fn courses(
    user: AdminUser,
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::Courses> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    let courses = service.list()?;

    Ok(templates::Courses {
        role: user.0.role,
        flash: flash.map(|f| (f.name().to_owned(), f.msg().into())),
        courses,
    })
}

/// Course creation form for administrators.
#[get("/new")]
pub fn new_course(
    user: AdminUser,
    flash: Option<FlashMessage<'_, '_>>,
    conn: DbConn,
) -> Result<templates::NewCourse> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    let (authors, tutors) = service.list_authors_tutors()?;

    Ok(templates::NewCourse {
        role: user.0.role,
        flash: flash.map(|f| f.msg().into()),
        authors,
        tutors,
    })
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
pub fn post_new_course(
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

/// Enable or disable courses as administrator.
#[get("/<id>/enable?<value>")]
pub fn enable_course(_user: AdminUser, id: i32, value: bool, conn: DbConn) -> Result<Redirect> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    service.enable(id, value)?;

    Ok(Redirect::to(uri!("/courses", courses)))
}

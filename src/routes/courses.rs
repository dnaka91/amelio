//! Course management related routes.

use anyhow::Result;
use log::error;
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use super::{NonEmptyString, PositiveId, ServerError};
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
) -> Result<templates::Courses, ServerError> {
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
) -> Result<templates::NewCourse, ServerError> {
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
    code: NonEmptyString,
    title: NonEmptyString,
    author: PositiveId,
    tutor: PositiveId,
}

/// New course POST endpoint to handle course creation, only for administrators.
#[post("/new", data = "<data>")]
pub fn post_new_course(_user: AdminUser, data: Form<NewCourse>, conn: DbConn) -> Flash<Redirect> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );

    match service.create(
        data.0.code.0,
        data.0.title.0,
        data.0.author.0,
        data.0.tutor.0,
    ) {
        Ok(()) => Flash::success(
            Redirect::to(uri!("/courses", courses)),
            MessageCode::CourseCreated,
        ),
        Err(e) => {
            error!("error during course creation: {:?}", e);
            Flash::error(
                Redirect::to(uri!("/courses", new_course)),
                MessageCode::FailedCourseCreation,
            )
        }
    }
}

/// Enable or disable courses as administrator.
#[get("/<id>/enable?<value>")]
pub fn enable_course(
    _user: AdminUser,
    id: PositiveId,
    value: bool,
    conn: DbConn,
) -> Result<Redirect, ServerError> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    service.enable(id.0, value)?;

    Ok(Redirect::to(uri!("/courses", courses)))
}

/// Course editing form for administrators.
#[get("/<id>/edit")]
pub fn edit_course(
    user: AdminUser,
    id: PositiveId,
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::EditCourse, ServerError> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );
    let course = service.get(id.0)?;
    let (authors, tutors) = service.list_authors_tutors()?;

    Ok(templates::EditCourse {
        role: user.0.role,
        flash: flash.map(|f| f.msg().into()),
        authors,
        tutors,
        course,
    })
}

/// Form data from the course editing form.
#[derive(FromForm)]
pub struct EditCourse {
    title: NonEmptyString,
    author: PositiveId,
    tutor: PositiveId,
}

/// Edit course POST endpoint to handle course editing, only for administrators.
#[post("/<id>/edit", data = "<data>")]
pub fn post_edit_course(
    _user: AdminUser,
    id: PositiveId,
    data: Form<EditCourse>,
    conn: DbConn,
) -> Flash<Redirect> {
    let service = services::course_service(
        repositories::user_repo(&conn),
        repositories::course_repo(&conn),
    );

    match service.update(id.0, data.0.title.0, data.0.author.0, data.0.tutor.0) {
        Ok(()) => Flash::success(
            Redirect::to(uri!("/courses", courses)),
            MessageCode::CourseUpdated,
        ),
        Err(e) => {
            error!("error during course update: {:?}", e);
            Flash::error(
                Redirect::to(format!("/courses/{}/edit", id.0)),
                MessageCode::FailedCourseUpdate,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::Status;
    use rocket::uri;

    use crate::tests::{check_form, prepare_logged_in_client};

    #[test]
    fn invalid_new_course() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/courses", super::post_new_course).to_string();

        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=&title=a&author=1&tutor=1").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&title=&author=1&tutor=1").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&title=a&author=0&tutor=1").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&title=a&author=1&tutor=0").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&title=a&author=&tutor=1").status()
        );
        assert_eq!(
            Status::UnprocessableEntity,
            check_form(&client, &uri, "code=a&title=a&author=1&tutor=").status()
        );
    }

    #[test]
    fn invalid_enable_course_id() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = "/courses/0/enable?value=true";

        assert_eq!(Status::NotFound, client.get(uri).dispatch().status());
    }
}

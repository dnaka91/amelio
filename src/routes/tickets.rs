//! Ticket related routes.

use anyhow::Result;
use log::error;
use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form, FormItems, FormParseError, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use super::{Hour, Minute, NonEmptyString, PositiveId, PositiveNum, Second, ServerError};
use crate::db::connection::DbConn;
use crate::db::repositories;
use crate::models::{Category, Id, TicketType};
use crate::roles::StudentUser;
use crate::services::{self, TicketService};
use crate::templates::{self, MessageCode};

/// Ticket listing page for students or higher ranked users.
#[get("/")]
pub fn tickets(
    user: StudentUser,
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::Tickets, ServerError> {
    let service = services::ticket_service(
        repositories::ticket_repo(&conn),
        repositories::course_repo(&conn),
    );
    let tickets = service.list()?;

    Ok(templates::Tickets {
        role: user.0.role,
        flash: flash.map(|f| (f.name().to_owned(), f.msg().into())),
        tickets,
    })
}

/// Ticket creation form for students or higher ranked users.
#[get("/new/<ty>")]
pub fn new_ticket(
    user: StudentUser,
    ty: TicketType,
    conn: DbConn,
) -> Result<templates::NewTicket, ServerError> {
    let service = services::ticket_service(
        repositories::ticket_repo(&conn),
        repositories::course_repo(&conn),
    );
    let courses = service.list_course_names()?;

    Ok(templates::NewTicket {
        role: user.0.role,
        ty,
        courses,
    })
}

/// Form data from the ticket creation form. It contains all available data for differnet kinds of
/// tickets and should never be used directly in a route. Instead use the [`NewTicket`] struct.
#[derive(FromForm)]
struct NewTicketData {
    ty: TicketType,
    category: Category,
    title: NonEmptyString,
    description: NonEmptyString,
    course: PositiveId,
    page: Option<PositiveNum<u16>>,
    line: Option<PositiveNum<u16>>,
    url: Option<NonEmptyString>,
    question: Option<PositiveNum<u16>>,
    answer: Option<NonEmptyString>,
    hour: Option<Hour>,
    minute: Option<Minute>,
    second: Option<Second>,
}

/// Form data for the ticket creation form.
pub struct NewTicket {
    ty: TicketType,
    category: Category,
    title: String,
    description: String,
    course: Id,
    medium: Medium,
}

/// Different kinds of media from the ticket creation form.
pub enum Medium {
    Text { page: u16, line: u16 },
    Interactive { url: String },
    Questionaire { question: u16, answer: String },
    Recording { hour: u8, minute: u8, second: u8 },
}

impl<'f> FromForm<'f> for NewTicket {
    type Error = rocket::request::FormParseError<'f>;

    fn from_form(it: &mut FormItems<'f>, strict: bool) -> Result<Self, Self::Error> {
        let missing = |name| FormParseError::Missing(RawStr::from_str(name));

        let data = NewTicketData::from_form(it, strict)?;
        let medium = match data.ty {
            TicketType::CourseBook | TicketType::ReadingList | TicketType::Presentation => {
                Medium::Text {
                    page: data.page.ok_or_else(|| missing("page"))?.0,
                    line: data.line.ok_or_else(|| missing("line"))?.0,
                }
            }
            TicketType::InteractiveBook => Medium::Interactive {
                url: data.url.ok_or_else(|| missing("url"))?.0,
            },
            TicketType::PracticeExam
            | TicketType::PracticeExamSolution
            | TicketType::OnlineTest => Medium::Questionaire {
                question: data.question.ok_or_else(|| missing("question"))?.0,
                answer: data.answer.ok_or_else(|| missing("answer"))?.0,
            },
            TicketType::Vodcast | TicketType::Podcast | TicketType::LiveTutorialRecording => {
                Medium::Recording {
                    hour: data.hour.ok_or_else(|| missing("hour"))?.0,
                    minute: data.minute.ok_or_else(|| missing("minute"))?.0,
                    second: data.second.ok_or_else(|| missing("second"))?.0,
                }
            }
        };

        Ok(Self {
            ty: data.ty,
            category: data.category,
            title: data.title.0,
            description: data.description.0,
            course: data.course.0,
            medium,
        })
    }
}

/// New ticket POST endpoint to handle ticket creation, for students or higher ranked users.
#[post("/new", data = "<data>")]
pub fn post_new_ticket(user: StudentUser, data: Form<NewTicket>, conn: DbConn) -> Flash<Redirect> {
    let service = services::ticket_service(
        repositories::ticket_repo(&conn),
        repositories::course_repo(&conn),
    );

    match service.create(
        crate::models::NewTicket {
            type_: data.ty,
            title: data.0.title,
            description: data.0.description,
            category: data.0.category,
            course_id: data.0.course,
            creator_id: user.0.id,
        },
        match data.0.medium {
            Medium::Text { page, line } => crate::models::NewMedium::Text { page, line },
            Medium::Recording {
                hour,
                minute,
                second,
            } => crate::models::NewMedium::Recording {
                time: chrono::NaiveTime::from_hms(hour.into(), minute.into(), second.into()),
            },
            Medium::Interactive { url } => crate::models::NewMedium::Interactive { url },
            Medium::Questionaire { question, answer } => {
                crate::models::NewMedium::Questionaire { question, answer }
            }
        },
    ) {
        Ok(()) => Flash::success(
            Redirect::to(uri!("/tickets", tickets)),
            MessageCode::TicketCreated,
        ),
        Err(e) => {
            error!("error during ticket creation: {:?}", e);
            Flash::error(
                Redirect::to(format!("/tickets/new/{}", data.0.ty)),
                MessageCode::FailedTicketCreation,
            )
        }
    }
}

/// Show the details of a ticket from student perspective.
#[get("/<id>")]
pub fn edit_ticket_student(
    user: StudentUser,
    id: PositiveId,
    conn: DbConn,
    flash: Option<FlashMessage<'_, '_>>,
) -> Result<templates::TicketDetail, ServerError> {
    let service = services::ticket_service(
        repositories::ticket_repo(&conn),
        repositories::course_repo(&conn),
    );

    let ticket = service.get_with_medium(id.0)?;

    Ok(templates::TicketDetail {
        role: user.0.role,
        flash: flash.map(|f| f.msg().into()),
        ticket,
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rocket::http::Status;
    use rocket::uri;

    use crate::tests::{check_form, prepare_logged_in_client};

    #[test]
    fn invalid_post_new_ticket() {
        let client = prepare_logged_in_client("admin", "admin");
        let uri = uri!("/tickets", super::post_new_ticket).to_string();

        let data_list = &[
            "ty=&category=content&title=a&description=a&course=1&page=1&line=1",
            "ty=course-book&category=&title=a&description=a&course=1&page=1&line=1",
            "ty=course-book&category=content&title=&description=a&course=1&page=1&line=1",
            "ty=course-book&category=content&title=a&description=&course=1&page=1&line=1",
            "ty=course-book&category=content&title=a&description=a&course=0&page=1&line=1",
            "ty=course-book&category=content&title=a&description=a&course=1&page=0&line=1",
            "ty=course-book&category=content&title=a&description=a&course=1&page=1&line=0",
            "ty=course-book&category=content&title=a&description=a&course=1&page=1&line=0",
            "ty=vodcast&category=content&title=a&description=a&course=1&hour=24&minute=0&second=0",
            "ty=vodcast&category=content&title=a&description=a&course=1&hour=0&minute=60&second=0",
            "ty=vodcast&category=content&title=a&description=a&course=1&hour=0&minute=0&second=60",
            "ty=interactive-book&category=content&title=a&description=a&course=1&url=",
            "ty=practice-exam&category=content&title=a&description=a&course=1&question=0&answer=a",
            "ty=practice-exam&category=content&title=a&description=a&course=1&question=1&answer=",
        ];

        for data in data_list {
            assert_eq!(
                Status::UnprocessableEntity,
                check_form(&client, &uri, data).status(),
                "data = {}",
                data
            );
        }
    }
}

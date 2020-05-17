//! Ticket related routes.

use anyhow::Result;
use log::error;
use rocket::http::RawStr;
use rocket::request::{FlashMessage, Form, FormItems, FormParseError, FromForm};
use rocket::response::{Flash, Redirect};
use rocket::{get, post, uri};

use super::{NonEmptyString, PositiveId, PositiveNum, ServerError};
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
    hour: Option<u8>,   // TODO
    minute: Option<u8>, // TODO
    second: Option<u8>, // TODO
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
                    hour: data.hour.ok_or_else(|| missing("hour"))?,
                    minute: data.minute.ok_or_else(|| missing("minute"))?,
                    second: data.second.ok_or_else(|| missing("second"))?,
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

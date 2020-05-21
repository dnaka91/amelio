//! The base models of the system, that [`services`](crate::services) work on.

use chrono::{DateTime, NaiveTime, Utc};
use strum::{AsRefStr, Display, EnumString};
use url::Url;

/// The identifier type for all models.
pub type Id = i32;

/// Different roles that a user can have.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Role {
    Admin,
    Author,
    Tutor,
    Student,
}

/// A full user with all available details.
pub struct User {
    pub id: Id,
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: Role,
    pub active: bool,
    pub code: String,
}

impl User {
    /// Check whether this user is the very first administrator.
    pub fn is_admin(&self) -> bool {
        self.id == 1 && self.role == Role::Admin
    }
}

/// A basic new user that is not part of the system yet.
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub role: Role,
    pub code: String,
}

/// An existing user to be updated.
pub struct EditUser {
    pub id: Id,
    pub name: String,
    pub role: Role,
}

/// A full course with all available details.
pub struct Course {
    pub id: Id,
    pub code: String,
    pub title: String,
    pub author_id: Id,
    pub tutor_id: Id,
    pub active: bool,
}

/// A new course to be added to the system.
pub struct NewCourse {
    pub code: String,
    pub title: String,
    pub author_id: Id,
    pub tutor_id: Id,
}

/// An existing course to be updated.
pub struct EditCourse {
    pub id: Id,
    pub title: String,
    pub author_id: Id,
    pub tutor_id: Id,
}

/// A course with its author and tutor names included.
pub struct CourseWithNames {
    pub course: Course,
    pub author_name: String,
    pub tutor_name: String,
}

/// Different types of [`Ticket`]s. It also decides what kind of medium is attached to a ticket.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TicketType {
    CourseBook,
    ReadingList,
    InteractiveBook,
    PracticeExam,
    PracticeExamSolution,
    Vodcast,
    Podcast,
    Presentation,
    LiveTutorialRecording,
    OnlineTest,
}

impl TicketType {
    pub fn medium(self) -> MediumType {
        match self {
            Self::CourseBook | Self::ReadingList | Self::Presentation => MediumType::Text,
            Self::Vodcast | Self::Podcast | Self::LiveTutorialRecording => MediumType::Recording,
            Self::InteractiveBook => MediumType::Interactive,
            Self::PracticeExam | Self::PracticeExamSolution | Self::OnlineTest => {
                MediumType::Questionaire
            }
        }
    }
}

/// The kind of medium that is attached to a [`Ticket`] based on the [`TicketType`].
pub enum MediumType {
    Text,
    Recording,
    Interactive,
    Questionaire,
}

/// The category allows to group [`Ticket`]s into specific topics.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Category {
    Editorial,
    Content,
    Improvement,
    Addition,
}

/// Different priorities of a [`Ticket`].
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

/// The current status of a ticket.
///
/// - A newly created ticket starts with the [`Status::Open`] state.
/// - The first time in the [`Status::Open`] state is opened by a tutor (or author), the status
///   changes to [`Status::InProgress`].
/// - Once a ticket was reviewed it can either become [`Status::Accepted`] or [`Status::Refused`].
///   If it was refused, then this is the final status.
/// - If the ticket is [`Status::Accepted`] it becomes [`Status::Completed`] once the related medium
///   was is updated. This is the final status.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Display, EnumString, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Status {
    Open,
    InProgress,
    Accepted,
    Refused,
    Completed,
}

/// A full ticket with all available details.
pub struct Ticket {
    pub id: Id,
    pub type_: TicketType,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub priority: Priority,
    pub status: Status,
    pub course_id: Id,
    pub creator_id: Id,
}

/// A ticket with its course and creator names included.
pub struct TicketWithNames {
    pub ticket: Ticket,
    pub course_name: String,
    pub creator_name: String,
}

/// A ticket with the same information as [`TicketWithNames`] plus the related medium.
pub struct TicketWithRels {
    pub ticket: Ticket,
    pub course_name: String,
    pub creator_name: String,
    pub medium: Medium,
    pub comments: Vec<CommentWithNames>,
}

/// A medium contains additional information to locate content for a [`Ticket`]. The specific type
/// depends on the [`TicketType`].
pub enum Medium {
    /// A medium which describes locations in text based content.
    ///
    /// This is the content for:
    /// - [`TicketType::CourseBook`]
    /// - [`TicketType::ReadingList`]
    /// - [`TicketType::Presentation`]
    Text { ticket_id: Id, page: u16, line: u16 },
    /// A medium which describes locations in recorded content like videos.
    ///
    /// This is the content for:
    /// - [`TicketType::Vodcast`]
    /// - [`TicketType::Podcast`]
    /// - [`TicketType::LiveTutorialRecording`]
    Recording { ticket_id: Id, time: NaiveTime },
    /// A medium which describes locations in interactive content like websites.
    ///
    /// This is the content for:
    /// - [`TicketType::InteractiveBook`]
    Interactive { ticket_id: Id, url: Url },
    /// A medium which describes locations in question-answer structured content like tests.
    ///
    /// This is the content for:
    /// - [`TicketType::PracticeExam`]
    /// - [`TicketType::PracticeExamSolution`]
    /// - [`TicketType::OnlineTest`]
    Questionaire {
        ticket_id: Id,
        question: u16,
        answer: String,
    },
}

/// A new ticket to be added to the system.
pub struct NewTicket {
    pub type_: TicketType,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub course_id: Id,
    pub creator_id: Id,
}

/// A new medium that belongs to a [`Ticket`] that is to be added to the system.
pub enum NewMedium {
    Text { page: u16, line: u16 },
    Recording { time: NaiveTime },
    Interactive { url: Url },
    Questionaire { question: u16, answer: String },
}

/// A full comment with all available details.
pub struct Comment {
    pub id: Id,
    pub ticket_id: Id,
    pub creator_id: Id,
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

/// A comment with its creator name included.
pub struct CommentWithNames {
    pub comment: Comment,
    pub creator_name: String,
}

/// A new comment to be added to the system.
pub struct NewComment {
    pub ticket_id: Id,
    pub creator_id: Id,
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

//! Language (mostly German) specific functionality.

use crate::models::{Category, Priority, Role, Status, TicketType};

/// The translate trait allows for any implementing object to translate itself or its value into
/// different languages.
pub trait Translate {
    /// Translate to German language.
    fn german(&self) -> &'static str;
}

impl Translate for Role {
    fn german(&self) -> &'static str {
        match self {
            Self::Admin => "Administrator",
            Self::Author => "Autor",
            Self::Tutor => "Tutor",
            Self::Student => "Student",
        }
    }
}

impl Translate for TicketType {
    fn german(&self) -> &'static str {
        match self {
            Self::CourseBook => "Skript",
            Self::ReadingList => "Literaturliste",
            Self::InteractiveBook => "Interactive Book",
            Self::PracticeExam => "Musterklausur",
            Self::PracticeExamSolution => "Musterl\u{00f6}sung",
            Self::Vodcast => "Vodcast",
            Self::Podcast => "Podcast",
            Self::Presentation => "Pr\u{00e4}sentation",
            Self::LiveTutorialRecording => "Live Tutorium Aufzeichnung",
            Self::OnlineTest => "Online Test",
        }
    }
}

impl Translate for Category {
    fn german(&self) -> &'static str {
        match self {
            Self::Editorial => "Redaktioneller Fehler",
            Self::Content => "Inhaltlicher Fehler",
            Self::Improvement => "Verbesserungsvorschlag",
            Self::Addition => "Erg\u{00e4}nzungsvorschlag",
        }
    }
}

impl Translate for Priority {
    fn german(&self) -> &'static str {
        match self {
            Self::Critical => "Kritisch",
            Self::High => "Hoch",
            Self::Medium => "Mittel",
            Self::Low => "Niedrig",
        }
    }
}

impl Translate for Status {
    fn german(&self) -> &'static str {
        match self {
            Self::Open => "Offen",
            Self::InProgress => "In Bearbeitung",
            Self::Accepted => "Aktzeptiert",
            Self::Refused => "Abgelehnt",
            Self::Completed => "Abgeschlossen",
        }
    }
}

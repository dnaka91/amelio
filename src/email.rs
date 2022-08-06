//! Functions for sending emails.

use anyhow::Result;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::error;

use crate::config::SmtpConfig;
use crate::language::Translate;
use crate::models::{Id, Status};

/// A mail sender allows to send emails.
pub trait MailSender {
    /// Send a single mail to one recipient with subject and a plain text message.
    fn send(&self, mail: Mail<'_>) -> Result<()>;
}

/// All information needed to send a single mail with the [`MailSender`].
pub struct Mail<'a> {
    pub from: (&'a str, &'a str),
    pub to: (&'a str, &'a str),
    pub subject: &'a str,
    pub message: &'a str,
}

/// Implementation of [`MailSender`] that uses a SMTP client to send emails.
struct SmtpSender<'a> {
    config: &'a SmtpConfig,
}

impl<'a> MailSender for SmtpSender<'a> {
    fn send(&self, mail: Mail<'_>) -> Result<()> {
        let sender = SmtpTransport::starttls_relay(&self.config.domain)?
            .port(self.config.port)
            .credentials(Credentials::new(
                self.config.username.clone(),
                self.config.password.clone(),
            ))
            .build();

        let email_message = Message::builder()
            .header(ContentType::TEXT_PLAIN)
            // Usually SMTP servers refuse to send emails when the **From** field doesn't match with
            // the actual user account. Therefore, we just ignore the username from [`Mail`] and use
            // the configuration's username instead.
            .from(Mailbox::new(
                Some(mail.from.1.to_owned()),
                self.config.username.parse()?,
            ))
            .to(Mailbox::new(Some(mail.to.1.to_owned()), mail.to.0.parse()?))
            .subject(mail.subject)
            .body(mail.message.to_owned())?;

        #[allow(clippy::match_wildcard_for_single_variants)]
        std::thread::spawn(move || match sender.send(&email_message) {
            Ok(r) if !r.is_positive() => error!("Failed sending email: {}", r.code()),
            Err(e) => error!("Failed sending email: {:?}", e),
            _ => (),
        });

        Ok(())
    }
}

/// Create a new mail sender that uses a SMTP client.
pub fn new_smtp_sender(config: &SmtpConfig) -> impl MailSender + '_ {
    SmtpSender { config }
}

/// A mail renderer creates the subject and body for emails of different purposes.
pub trait MailRenderer {
    /// Create the invitation email for account activation.
    fn invitation(&self, name: &str, code: &str) -> (&str, String);
    /// Create the status change email for whenever a ticket status changes.
    fn status_change(&self, name: &str, details: StatusDetails<'_>) -> (&str, String);
    /// Create the new comment email for whenever someone adds a new comment to a ticket.
    fn new_comment(&self, name: &str, details: CommentDetails<'_>) -> (&str, String);
}

/// Detail information to create the status change email.
pub struct StatusDetails<'a> {
    pub ticket_title: &'a str,
    pub ticket_id: Id,
    pub old_status: Status,
    pub new_status: Status,
}

/// Detail information to create the new comment email.
pub struct CommentDetails<'a> {
    pub ticket_title: &'a str,
    pub ticket_id: Id,
    pub comment: &'a str,
    pub writer_name: &'a str,
}

/// Main implementation of [`MailRenderer`].
struct MailRendererImpl<'a> {
    host: &'a str,
}

impl<'a> MailRenderer for MailRendererImpl<'a> {
    fn invitation(&self, name: &str, code: &str) -> (&str, String) {
        (
            "Amelio Registrierung",
            format!(
                "Hallo {},\n\
                \n\
                Willkommen bei Amelio!\n\
                \n\
                Bitte clicke auf den folgenden Link um Deinen Account zu aktivieren:\n\
                {}/activate/{}\n\
                \n\
                Viele Gr\u{00fc}\u{00df}e,\n\
                Dein Amelio-Team",
                name, self.host, code,
            ),
        )
    }

    fn status_change(&self, name: &str, details: StatusDetails<'_>) -> (&str, String) {
        (
            "Status\u{00e4}nderung Deines Tickets",
            format!(
                "Hallo {name},\n\
                \n\
                Der Status Deines Tickets \"{title}\" wurde soeben von {old} zu {new} \
                ge\u{00e4}ndert.\n\
                \n\
                Du kannst dein Ticket jederzeit unter folgendem Link einsehen:\n\
                {host}/tickets/{id}\n\
                \n\
                Viele Gr\u{00fc}\u{00df}e,\n\
                Dein Amelio-Team",
                name = name,
                title = details.ticket_title,
                old = details.old_status.german(),
                new = details.new_status.german(),
                host = self.host,
                id = details.ticket_id
            ),
        )
    }

    fn new_comment(&self, name: &str, details: CommentDetails<'_>) -> (&str, String) {
        (
            "Neuer Kommentar f\u{00fc}r Dein Ticket",
            format!(
                "Hallo {name},\n\
                \n\
                Deinem Ticket \"{title}\" wurde soeben ein neuer Komentar von {writer} \
                hinzugef\u{00fc}gt:\n\
                \n\
                {comment}\n\
                \n\
                Du kannst dein Ticket jederzeit unter folgendem Link einsehen:\n\
                {host}/tickets/{id}\n\
                \n\
                Viele Gr\u{00fc}\u{00df}e,\n\
                Dein Amelio-Team",
                name = name,
                title = details.ticket_title,
                writer = details.writer_name,
                comment = details.comment,
                host = self.host,
                id = details.ticket_id,
            ),
        )
    }
}

/// Create a new mail renderer.
pub fn new_mail_renderer(host: &str) -> impl MailRenderer + '_ {
    MailRendererImpl { host }
}

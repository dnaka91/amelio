//! Functions for sending emails.

use anyhow::{ensure, Result};
use lettre::smtp::authentication::Credentials;
use lettre::{ClientSecurity, ClientTlsParameters, SmtpClient, Transport};
use lettre_email::EmailBuilder;
use native_tls::{Protocol, TlsConnector};

use crate::config::SmtpConfig;

/// A mail sender allows to send emails.
pub trait MailSender {
    /// Send a single mail to one recipient with subject and a plain text message.
    fn send(&self, mail: Mail) -> Result<()>;
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

impl<'a> SmtpSender<'a> {
    /// Create and configure a SMTP client.
    ///
    /// Currently the SMTP client builder of [`lettre`] doesn't allow to set a port for connecting.
    /// The unreleased version `0.10` of [`lettre`] already includes that feature and once released
    /// this function can be removed.
    fn create_client(domain: &str, port: u16) -> Result<SmtpClient> {
        let mut tls_builder = TlsConnector::builder();
        tls_builder.min_protocol_version(Some(Protocol::Tlsv12));

        let params = ClientTlsParameters::new(domain.to_owned(), tls_builder.build()?);

        SmtpClient::new((domain, port), ClientSecurity::Required(params)).map_err(Into::into)
    }
}

impl<'a> MailSender for SmtpSender<'a> {
    fn send(&self, mail: Mail) -> Result<()> {
        let mut sender = Self::create_client(&self.config.domain, self.config.port)?
            .credentials(Credentials::new(
                self.config.username.clone(),
                self.config.password.clone(),
            ))
            .transport();

        let email = EmailBuilder::new()
            // Usually SMTP servers refuse to send emails when the **From** field doesn't match with
            // the actual user account. Therefore, we just ignore the username from [`Mail`] and use
            // the configuration's username instead.
            .from((&self.config.username, "Amelio"))
            .to(mail.to)
            .subject(mail.subject)
            .text(mail.message)
            .build()?;

        let resp = sender.send(email.into())?;

        ensure!(resp.is_positive(), "Failed sending email: {}", resp.code);
        Ok(())
    }
}

/// Create a new mail sender that uses a SMTP client.
pub fn new_smtp_sender<'a>(config: &'a SmtpConfig) -> impl MailSender + 'a {
    SmtpSender { config }
}

/// A mail renderer creates the subject and body for emails of different purposes.
pub trait MailRenderer {
    /// Create the invitation email for account activation.
    fn invitation(&self, name: &str, code: &str) -> (&str, String);
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
                {}/users/activate/{}\n\
                \n\
                Viele Gr\u{00fc}\u{00df}e,\n\
                Dein Amelio-Team",
                name, self.host, code,
            ),
        )
    }
}

/// Create a new mail renderer.
pub fn new_mail_renderer<'a>(host: &'a str) -> impl MailRenderer + 'a {
    MailRendererImpl { host }
}

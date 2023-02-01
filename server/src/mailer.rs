use crate::config::{EmailConfig, EmailSecurity};
use lettre::message::{Mailbox, MessageBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use std::error::Error;

pub struct Mailer {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new(config: EmailConfig) -> Result<Mailer, Box<dyn Error>> {
        let tls_parameters = TlsParameters::new(config.smtp_server_address.clone())?;
        let tls = match config.smtp_security {
            EmailSecurity::TLS => Tls::Wrapper(tls_parameters),
            EmailSecurity::StartTLS => Tls::Required(tls_parameters),
            EmailSecurity::None => Tls::None,
        };
        let mailer =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(config.smtp_server_address)
                .port(config.smtp_server_port)
                .tls(tls)
                .credentials(Credentials::new(config.smtp_username, config.smtp_password))
                .build();
        Ok(Mailer { mailer })
    }

    pub async fn send_email(
        &self,
        to: String,
        from_name: String,
        from_email: String,
        subject: String,
        body: String,
    ) -> Result<(), Box<dyn Error>> {
        self.mailer
            .send(
                MessageBuilder::new()
                    .to(Mailbox::new(None, to.parse()?))
                    .from(Mailbox::new(Some(from_name), from_email.parse()?))
                    .subject(subject)
                    .body(body)?,
            )
            .await?;
        Ok(())
    }
}

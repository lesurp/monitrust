use anyhow::Result;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::{Message, SendmailTransport, Transport};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::alert_reporter::AlertReporter;
use crate::watcher::ActiveAlert;

pub struct Mail {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    from: String,
    to: String,
}

impl Mail {
    pub fn try_new(configuration: Configuration) -> Result<Self> {
        configuration.from.parse::<Mailbox>()?;
        configuration.to.parse::<Mailbox>()?;
        Ok(Self {
            from: configuration.from,
            to: configuration.to,
        })
    }
}

impl AlertReporter for Mail {
    fn report(&self, alert: &ActiveAlert) -> Result<()> {
        info!(reporting = "mail");

        let email = Message::builder()
            .from(self.from.parse()?)
            .to(self.to.parse()?)
            .subject(&alert.message)
            .header(ContentType::TEXT_PLAIN)
            .body(alert.message.clone())
            .unwrap();

        let mailer = SendmailTransport::new();
        mailer.send(&email)?;
        Ok(())
    }
}

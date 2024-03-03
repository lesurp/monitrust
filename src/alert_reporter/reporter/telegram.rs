use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use teloxide::Bot;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::Recipient;
use tracing::info;

use crate::alert_reporter::AlertReporter;
use crate::watcher::ActiveAlert;

pub struct Telegram {
    bot: Bot,
    chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    api: String,
    chat_id: i64,
}

impl Telegram {
    pub fn new(configuration: Configuration) -> Self {
        let bot = Bot::new(configuration.api);
        Telegram { bot, chat_id: configuration.chat_id }
    }
}

impl AlertReporter for Telegram {
    fn report(&self, alert: &ActiveAlert) -> Result<()> {
        info!(reporting = "telegram");
        tokio::runtime::Runtime::new().context("Could not create tokio runtime.")?.block_on(async {
            self.bot.send_message(Recipient::Id(ChatId(self.chat_id)), &alert.message).await.with_context(|| format!("Could not send message to chat id {}", self.chat_id)).map(|_| ())
        })
    }
}
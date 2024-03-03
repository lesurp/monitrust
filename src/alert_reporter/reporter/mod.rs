use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::alert_reporter::AlertReporter;
use crate::alert_reporter::reporter::telegram::Telegram;
use crate::watcher::ActiveAlert;

mod telegram;

pub enum Reporter {
    Telegram(Telegram),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AlertTargetConfiguration {
    Telegram(telegram::Configuration),
}

impl Into<Reporter> for AlertTargetConfiguration {
    fn into(self) -> Reporter {
        match self {
            AlertTargetConfiguration::Telegram(t) => Reporter::Telegram(Telegram::new(t)),
        }
    }
}

impl AlertReporter for Reporter {
    fn report(&self, alert: &ActiveAlert) -> Result<()> {
        match self {
            Reporter::Telegram(t) => t.report(alert),
        }
    }
}
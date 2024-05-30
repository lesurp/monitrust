use crate::watcher::ActiveAlert;
use anyhow::Result;
use enum_dispatch::enum_dispatch;
#[cfg(feature = "telegram")]
use reporters::telegram::{self, Telegram};
use serde::{Deserialize, Serialize};
use tracing::warn;
pub mod reporters;

#[enum_dispatch(Reporter)]
pub trait AlertReporter {
    fn report(&self, alert: &ActiveAlert) -> Result<()>;
}

pub struct MultiReporter(pub Vec<Reporter>);

impl AlertReporter for MultiReporter {
    fn report(&self, alert: &ActiveAlert) -> Result<()> {
        self.0
            .iter()
            .map(|a| a.report(alert))
            .filter_map(|r| match r {
                Ok(_) => None,
                Err(e) => Some(e),
            })
            .for_each(|e| {
                warn!(reporter_error = ?e);
            });
        Ok(())
    }
}

#[enum_dispatch]
pub enum Reporter {
    #[cfg(feature = "telegram")]
    Telegram,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AlertTargetConfiguration {
    #[cfg(feature = "telegram")]
    Telegram(telegram::Configuration),
}

impl From<AlertTargetConfiguration> for Reporter {
    fn from(value: AlertTargetConfiguration) -> Self {
        match value {
            AlertTargetConfiguration::Telegram(t) => Reporter::Telegram(Telegram::new(t)),
        }
    }
}

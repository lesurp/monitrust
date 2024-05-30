use std::time::Duration;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::watcher;
use crate::watcher::ActiveAlert;

#[derive(Debug)]
pub struct Checker {
    period: Duration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    period_hours: u64,
}

impl watcher::Checker for Checker {
    type CheckResult = ();
    type Configuration = Configuration;

    fn check(&self) -> Result<Self::CheckResult> {
        Ok(())
    }

    fn period(&self) -> Duration {
        self.period
    }

    fn new(configuration: Self::Configuration) -> Self {
        Checker {
            period: Duration::from_secs(3600 * configuration.period_hours),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {}

impl watcher::Alert for Alert {
    type Checker = Checker;

    fn is_triggered(
        &self,
        _: &<Self::Checker as watcher::Checker>::CheckResult,
    ) -> Option<ActiveAlert> {
        Some(ActiveAlert {
            message: "MonitRust is still running 💓".to_string(),
        })
    }
}

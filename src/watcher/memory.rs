use std::time::Duration;

use anyhow::{Context, Result};
use nix::sys::sysinfo::sysinfo;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::watcher;
use crate::watcher::ActiveAlert;

// TODO: why does the compiler complain about that?
#[derive(Debug)]
pub struct Checker {
    period: Duration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    period_minutes: u64,
}

impl watcher::Checker for Checker {
    type CheckResult = f64;
    type Configuration = Configuration;

    fn check(&self) -> Result<Self::CheckResult> {
        info!(checking = "memory");
        let sysinfo = sysinfo().context("Could not execute 'sysinfo'")?;
        let free_memory = (sysinfo.ram_unused() as f64) / (sysinfo.ram_total() as f64);
        info!(free_memory);
        Ok(free_memory)
    }

    fn period(&self) -> Duration {
        self.period
    }

    fn new(configuration: Self::Configuration) -> Self {
        Checker {
            period: Duration::from_secs(60 * configuration.period_minutes),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    min: f64,
    max: f64,
}

impl watcher::Alert for Alert {
    type Checker = Checker;

    fn is_triggered(
        &self,
        check_result: &<Self::Checker as watcher::Checker>::CheckResult,
    ) -> Option<ActiveAlert> {
        if self.min < *check_result && *check_result < self.max {
            Some(ActiveAlert {
                message: format!(
                    "Memory usage is at {:.2}% (threshold: {:.2}%).",
                    100.0 * *check_result,
                    100.0 * self.max
                ),
            })
        } else {
            None
        }
    }
}

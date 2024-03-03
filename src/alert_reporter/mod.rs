use anyhow::Result;
use tracing::warn;

use crate::watcher::ActiveAlert;

pub mod reporter;

pub trait AlertReporter {
    fn report(&self, alert: &ActiveAlert) -> Result<()>;
}

pub struct MultiReporter(pub Vec<reporter::Reporter>);

impl AlertReporter for MultiReporter {
    fn report(&self, alert: &ActiveAlert) -> Result<()> {
        self.0.iter().map(|a| a.report(&alert)).filter_map(|r| match r {
            Ok(_) => None,
            Err(e) => Some(e)
        }).for_each(|e| {
            warn!(reporter_error = ?e);
        });
        Ok(())
    }
}
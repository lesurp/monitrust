use std::io::{BufRead, BufReader};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

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
        let meminfo_file = "/proc/meminfo";
        let file = std::fs::File::open(meminfo_file)
            .context(format!("Could not open meminfo file at '{}'", meminfo_file))?;
        let buf_read = BufReader::new(file);
        const MEM_TOTAL_STR: &str = "MemTotal:";
        const MEM_AVAIL_STR: &str = "MemAvailable:";
        let mut mem_total = None;
        let mut mem_avail = None;
        for l in buf_read.lines() {
            let l = l.context("Could not read line from meminfo file.")?;
            let split = l.split_whitespace().collect::<Vec<_>>();
            match &split[..] {
                [name, value, _unit] => {
                    let target = if *name == MEM_TOTAL_STR {
                        &mut mem_total
                    } else if *name == MEM_AVAIL_STR {
                        &mut mem_avail
                    } else {
                        continue;
                    };
                    *target = Some(
                        value
                            .parse::<u64>()
                            .context(format!("Could not convert memory to integer: {}", value))?,
                    );
                }
                // Some values are just numbers e.g. page numbers
                [_, _alue] => {}
                _ => warn!("Could not parse line in meminfo file: {}", l),
            }
        }
        match (mem_total, mem_avail) {
            (Some(t), Some(a)) => {
                let free_memory = (a as f64) / (t as f64);
                info!(free_memory);
                Ok(free_memory)
            }
            (Some(_), None) => Err(anyhow!(
                "Could not retrieve available memory from meminfo file (key: {})",
                MEM_AVAIL_STR
            )),
            (None, Some(_)) => Err(anyhow!(
                "Could not retrieve total memory from meminfo file (key: {})",
                MEM_TOTAL_STR
            )),
            (None, None) => Err(anyhow!(
                "Could retrieve neither total nor available memory from meminfo file (keys: {} and {})",
                MEM_TOTAL_STR, MEM_AVAIL_STR
            )),
        }
        //let sysinfo = sysinfo().context("Could not execute 'sysinfo'")?;
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
                    "📝 Free memory is at {:.2}% (threshold: {:.2}%).",
                    100.0 * *check_result,
                    100.0 * self.max
                ),
            })
        } else {
            None
        }
    }
}

#![feature(thread_sleep_until)]

use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep_until;
use std::time::Instant;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use tracing::{error, info};

use crate::alert_reporter::MultiReporter;
use crate::alert_reporter::reporter::{AlertTargetConfiguration, Reporter};
use crate::scheduled_watcher::ScheduledWatcher;
use crate::watcher::{Watcher, WatcherConfiguration, WatcherEnum};

#[serde_with::serde_as]
#[derive(Deserialize)]
struct WatcherConfigurations(
    #[serde_as(as = "serde_with::EnumMap")]
    Vec<WatcherConfiguration>,
);

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct TargetConfigurations(
    #[serde_as(as = "serde_with::EnumMap")]
    Vec<AlertTargetConfiguration>,
);

mod watcher;
mod alert_reporter;
mod scheduled_watcher;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let watchers: HashSet<WatcherConfiguration> = {
        let watchers_file = "watchers.json";
        let file = File::open(watchers_file).with_context(|| format!("Could not open file: {}", watchers_file))?;
        let buf_reader = BufReader::new(file);
        let configurations: WatcherConfigurations = serde_json::from_reader(buf_reader)?;
        configurations.0.into_iter().collect()
    };
    info!(?watchers);

    let watchers: Vec<_> = watchers.into_iter().map(Into::<WatcherEnum>::into).collect();

    let targets: Vec<Reporter> = {
        let targets_file = "targets.json";
        let file = File::open(targets_file).with_context(|| format!("Could not open file: {}", targets_file))?;
        let buf_reader = BufReader::new(file);
        let configurations: TargetConfigurations = serde_json::from_reader(buf_reader)?;
        configurations.0.into_iter().map(Into::<Reporter>::into).collect()
    };
    let targets = MultiReporter(targets);

    let mut timers = BinaryHeap::new();

    let now = Instant::now();
    for w in watchers {
        timers.push(ScheduledWatcher { deadline: now, watcher: w });
    }

    while let Some(next) = timers.pop() {
        sleep_until(next.deadline);
        if let Err(e) = next.watcher.run(&targets) {
            error!(watcher = ?e);
        }
        timers.push(ScheduledWatcher { deadline: Instant::now() + next.watcher.period(), watcher: next.watcher });
    }

    Ok(())
}

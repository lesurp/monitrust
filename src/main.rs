#![feature(thread_sleep_until)]

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep_until;
use std::time::Instant;

use anyhow::Context;
use serde::Deserialize;
use serde::Serialize;
use tracing::error;

use crate::alert_reporter::{AlertTargetConfiguration, MultiReporter, Reporter};
use crate::scheduled_watcher::ScheduledWatcher;
use crate::watcher::{Watcher, WatcherConfiguration, WatcherEnum};

#[serde_with::serde_as]
#[derive(Deserialize)]
struct WatcherConfigurations(#[serde_as(as = "serde_with::EnumMap")] Vec<WatcherConfiguration>);

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
struct TargetConfigurations(#[serde_as(as = "serde_with::EnumMap")] Vec<AlertTargetConfiguration>);

mod alert_reporter;
mod scheduled_watcher;
mod watcher;

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let watchers = {
        let watchers_file = "watchers.json";
        let file = File::open(watchers_file)
            .with_context(|| format!("Could not open file: {}", watchers_file))?;
        let buf_reader = BufReader::new(file);
        let configurations: WatcherConfigurations = serde_json::from_reader(buf_reader)?;
        configurations
            .0
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .map(Into::<WatcherEnum>::into)
            .collect::<Vec<_>>()
    };

    let reporters = {
        let reporter_file = "reporters.json";
        let file = File::open(reporter_file)
            .with_context(|| format!("Could not open file: {}", reporter_file))?;
        let buf_reader = BufReader::new(file);
        let configurations: TargetConfigurations = serde_json::from_reader(buf_reader)?;
        MultiReporter(
            configurations
                .0
                .into_iter()
                .map(Into::<Reporter>::into)
                .collect(),
        )
    };

    let mut timers = BinaryHeap::new();

    let now = Instant::now();
    for w in watchers {
        timers.push(Reverse(ScheduledWatcher {
            deadline: now,
            watcher: w,
        }));
    }

    while let Some(Reverse(next)) = timers.pop() {
        sleep_until(next.deadline);
        if let Err(e) = next.watcher.run(&reporters) {
            error!(watcher = ?e);
        }
        timers.push(Reverse(ScheduledWatcher {
            deadline: Instant::now() + next.watcher.period(),
            watcher: next.watcher,
        }));
    }

    Ok(())
}

use std::cmp::Ordering;
use std::time::Instant;

use crate::watcher::WatcherEnum;

pub struct ScheduledWatcher {
    pub deadline: Instant,
    pub watcher: WatcherEnum,
}

impl Eq for ScheduledWatcher {}

impl PartialEq<Self> for ScheduledWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.deadline == other.deadline
    }
}

impl PartialOrd<Self> for ScheduledWatcher {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledWatcher {
    fn cmp(&self, other: &Self) -> Ordering {
        self.deadline.cmp(&other.deadline)
    }
}

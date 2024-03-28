use std::time::Duration;

use surrealdb_core::dbs::Response;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Stats {
    pub execution_time: Duration,
}

pub(super) trait ToStats {
    fn to_stats(&self) -> Stats;
}

impl ToStats for Response {
    fn to_stats(&self) -> Stats {
        Stats {
            execution_time: self.time,
        }
    }
}

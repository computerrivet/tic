use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A single task in the task list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub done: bool,
    /// Unix timestamp (seconds) when the task was created.
    ///
    /// Defaults to `0` for tasks persisted by older versions of the app,
    /// which did not record a creation time.
    #[serde(default)]
    pub created_at: i64,
}

impl Task {
    /// Create a new, not-yet-done task with the given id.
    pub fn new(id: usize, text: String) -> Self {
        Self {
            id,
            text,
            done: false,
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
        }
    }
}

/// Format a duration since `now` as a human-friendly relative time,
/// e.g. "1 min ago".
pub fn format_relative(created_at: i64, now: i64) -> String {
    if created_at <= 0 {
        return "unknown".to_string();
    }

    let secs = now.saturating_sub(created_at);
    let dur = Duration::from_secs(secs as u64);

    if dur.as_secs() < 60 {
        format!("{} sec ago", dur.as_secs())
    } else if dur.as_secs() < 3600 {
        format!("{} min ago", dur.as_secs() / 60)
    } else if dur.as_secs() < 86_400 {
        format!("{} hr ago", dur.as_secs() / 3600)
    } else {
        format!("{} days ago", dur.as_secs() / 86_400)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_starts_undone() {
        let task = Task::new(1, "write tests".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.text, "write tests");
        assert!(!task.done);
        assert!(task.created_at > 0);
    }

    #[test]
    fn format_relative_seconds() {
        assert_eq!(format_relative(1000, 1000), "0 sec ago");
        assert_eq!(format_relative(1000, 1001), "1 sec ago");
        assert_eq!(format_relative(1000, 1059), "59 sec ago");
    }

    #[test]
    fn format_relative_minutes() {
        assert_eq!(format_relative(1000, 1060), "1 min ago");
        assert_eq!(format_relative(1000, 1120), "2 min ago");
    }

    #[test]
    fn format_relative_hours() {
        assert_eq!(format_relative(1000, 1000 + 3600), "1 hr ago");
    }

    #[test]
    fn format_relative_days() {
        assert_eq!(format_relative(1000, 1000 + 2 * 86_400), "2 days ago");
    }

    #[test]
    fn format_relative_unknown() {
        assert_eq!(format_relative(0, 1000), "unknown");
    }
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

/// Priority buckets exposed to API clients.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "low"),
            TaskPriority::Medium => write!(f, "medium"),
            TaskPriority::High => write!(f, "high"),
        }
    }
}

impl FromStr for TaskPriority {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "low" => Ok(TaskPriority::Low),
            "medium" => Ok(TaskPriority::Medium),
            "high" => Ok(TaskPriority::High),
            _ => Err("invalid task priority"),
        }
    }
}

/// Task kanban states.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}

impl FromStr for TaskStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Err("invalid task status"),
        }
    }
}

/// Database model mirroring the `tasks` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub context_tags: serde_json::Value,
    pub github_url: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_priority_round_trip() {
        for (raw, variant) in [
            ("low", TaskPriority::Low),
            ("medium", TaskPriority::Medium),
            ("high", TaskPriority::High),
        ] {
            assert_eq!(TaskPriority::from_str(raw).unwrap(), variant);
            assert_eq!(variant.to_string(), raw);
        }
    }

    #[test]
    fn task_status_round_trip() {
        for (raw, variant) in [
            ("todo", TaskStatus::Todo),
            ("in_progress", TaskStatus::InProgress),
            ("done", TaskStatus::Done),
        ] {
            assert_eq!(TaskStatus::from_str(raw).unwrap(), variant);
            assert_eq!(variant.to_string(), raw);
        }
    }
}

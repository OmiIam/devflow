use crate::models::task::{Task, TaskPriority, TaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    #[serde(default = "TaskPriority::default_medium")]
    pub priority: TaskPriority,
    #[serde(default = "TaskStatus::default_todo")]
    pub status: TaskStatus,
    #[validate(custom = "validate_tags")]
    #[serde(default)]
    pub context_tags: Vec<String>,
    #[validate(url)]
    pub github_url: Option<String>,
    pub github_issue_number: Option<i32>,
    pub github_repo: Option<String>,
    #[validate(range(min = 1))]
    pub estimated_minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: Option<String>,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub priority: Option<TaskPriority>,
    pub status: Option<TaskStatus>,
    #[validate(custom = "validate_tags")]
    pub context_tags: Option<Vec<String>>,
    #[validate(url)]
    pub github_url: Option<String>,
    pub github_issue_number: Option<i32>,
    pub github_repo: Option<String>,
    #[validate(range(min = 1))]
    pub estimated_minutes: Option<i32>,
}

fn validate_tags(tags: &Vec<String>) -> Result<(), ValidationError> {
    for t in tags {
        if t.trim().is_empty() || t.len() > 50 {
            return Err(ValidationError::new("invalid_context_tag"));
        }
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub context_tags: Vec<String>,
    pub github_url: Option<String>,
    pub github_issue_number: Option<i32>,
    pub github_repo: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            user_id: task.user_id,
            title: task.title,
            description: task.description,
            priority: task.priority,
            status: task.status,
            context_tags: task.context_tags,
            github_url: task.github_url,
            github_issue_number: task.github_issue_number,
            github_repo: task.github_repo,
            estimated_minutes: task.estimated_minutes,
            completed_at: task.completed_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub data: Vec<TaskResponse>,
    pub meta: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl PaginationMeta {
    pub fn new(total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total + per_page - 1) / per_page;
        Self {
            total,
            page,
            per_page,
            total_pages,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskFilters {
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
    pub context_tag: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TaskSort {
    #[serde(default)]
    pub field: TaskSortField,
    #[serde(default)]
    pub order: SortOrder,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TaskSortField {
    #[default]
    CreatedAt,
    UpdatedAt,
    Priority,
}

impl std::fmt::Display for TaskSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSortField::CreatedAt => write!(f, "created_at"),
            TaskSortField::UpdatedAt => write!(f, "updated_at"),
            TaskSortField::Priority => write!(f, "priority"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

impl TaskPriority {
    pub fn default_medium() -> Self {
        TaskPriority::Medium
    }
}

impl TaskStatus {
    pub fn default_todo() -> Self {
        TaskStatus::Todo
    }
}

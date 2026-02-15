use crate::{
    dto::task::{CreateTaskRequest, TaskFilters, TaskSort, UpdateTaskRequest},
    models::task::Task,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct TaskRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TaskRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        request: CreateTaskRequest,
    ) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (
                user_id, title, description, priority, status, context_tags,
                github_url, github_issue_number, github_repo, estimated_minutes
            )
            VALUES ($1, $2, $3, $4, $5, to_jsonb($6::text[]),
                    $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(request.priority.to_string())
        .bind(request.status.to_string())
        .bind(&request.context_tags)
        .bind(&request.github_url)
        .bind(request.github_issue_number)
        .bind(&request.github_repo)
        .bind(request.estimated_minutes)
        .fetch_one(self.pool)
        .await
    }

    pub async fn find_by_id(
        &self,
        task_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Task>, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_optional(self.pool)
        .await
    }

    pub async fn find_all(
        &self,
        user_id: Uuid,
        filters: TaskFilters,
        sort: TaskSort,
        page: i64,
        per_page: i64,
    ) -> Result<(Vec<Task>, i64), sqlx::Error> {
        let offset = (page - 1) * per_page;

        let tasks = sqlx::query_as::<_, Task>(
            r#"
            SELECT * FROM tasks
            WHERE user_id = $1
              AND deleted_at IS NULL
              AND ($2::text IS NULL OR status = $2)
              AND ($3::text IS NULL OR priority = $3)
              AND (
                    $4::text IS NULL
                    OR context_tags @> to_jsonb(ARRAY[$4]::text[])
                  )
              AND (
                    $5::text IS NULL
                    OR title ILIKE '%' || $5 || '%'
                    OR description ILIKE '%' || $5 || '%'
                  )
            ORDER BY
              CASE WHEN $6 = 'created_at' AND $7 = 'asc' THEN created_at END ASC,
              CASE WHEN $6 = 'created_at' AND $7 = 'desc' THEN created_at END DESC,
              CASE WHEN $6 = 'updated_at' AND $7 = 'asc' THEN updated_at END ASC,
              CASE WHEN $6 = 'updated_at' AND $7 = 'desc' THEN updated_at END DESC,
              CASE WHEN $6 = 'priority' AND $7 = 'asc' THEN priority END ASC,
              CASE WHEN $6 = 'priority' AND $7 = 'desc' THEN priority END DESC
            LIMIT $8 OFFSET $9
            "#,
        )
        .bind(user_id)
        .bind(filters.status.map(|s| s.to_string()))
        .bind(filters.priority.map(|p| p.to_string()))
        .bind(filters.context_tag)
        .bind(filters.search)
        .bind(sort.field.to_string())
        .bind(sort.order.to_string())
        .bind(per_page)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        let total = sqlx::query_scalar::<_, i64>(
            r#"SELECT COUNT(*) FROM tasks WHERE user_id = $1 AND deleted_at IS NULL"#,
        )
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        Ok((tasks, total))
    }

    pub async fn update(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        request: UpdateTaskRequest,
    ) -> Result<Task, sqlx::Error> {
        let now = Utc::now();
        sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET title = COALESCE($3, title),
                description = COALESCE($4, description),
                priority = COALESCE($5, priority),
                status = COALESCE($6, status),
                context_tags = COALESCE(
                    CASE WHEN $7 IS NULL THEN NULL
                         ELSE to_jsonb($7::text[])
                    END,
                    context_tags
                ),
                github_url = COALESCE($8, github_url),
                github_issue_number = COALESCE($9, github_issue_number),
                github_repo = COALESCE($10, github_repo),
                estimated_minutes = COALESCE($11, estimated_minutes),
                completed_at = CASE
                    WHEN $6 = 'done' THEN COALESCE(completed_at, $12)
                    WHEN $6 IS NOT NULL AND $6 <> 'done' THEN NULL
                    ELSE completed_at
                END,
                updated_at = $12
            WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .bind(request.title)
        .bind(request.description)
        .bind(request.priority.map(|p| p.to_string()))
        .bind(request.status.map(|s| s.to_string()))
        .bind(request.context_tags)
        .bind(request.github_url)
        .bind(request.github_issue_number)
        .bind(request.github_repo)
        .bind(request.estimated_minutes)
        .bind(now)
        .fetch_one(self.pool)
        .await
    }

    pub async fn soft_delete(&self, task_id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query::<_>(
            r#"
            UPDATE tasks
            SET deleted_at = NOW()
            WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn restore(&self, task_id: Uuid, user_id: Uuid) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET deleted_at = NULL
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .fetch_one(self.pool)
        .await
    }

    pub async fn permanent_delete(&self, task_id: Uuid, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query::<_>(
            r#"
            DELETE FROM tasks
            WHERE id = $1 AND user_id = $2
            "#,
        )
        .bind(task_id)
        .bind(user_id)
        .execute(self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}

use crate::{
    dto::task::{
        CreateTaskRequest, PaginationMeta, TaskFilters, TaskListResponse, TaskResponse, TaskSort,
        UpdateTaskRequest,
    },
    repositories::task_repository::TaskRepository,
    utils::error::AppError,
};
use uuid::Uuid;
use validator::Validate;

const MAX_PAGE_SIZE: i64 = 100;

pub struct TaskService<'a> {
    repo: TaskRepository<'a>,
}

impl<'a> TaskService<'a> {
    pub fn new(repo: TaskRepository<'a>) -> Self {
        Self { repo }
    }

    pub async fn create_task(
        &self,
        request: CreateTaskRequest,
        user_id: Uuid,
    ) -> Result<TaskResponse, AppError> {
        request
            .validate()
            .map_err(|err| AppError::Validation(err.to_string()))?;

        let task = self
            .repo
            .create(user_id, request)
            .await
            .map_err(AppError::from)?;

        Ok(task.into())
    }

    pub async fn get_task(&self, task_id: Uuid, user_id: Uuid) -> Result<TaskResponse, AppError> {
        let task = self
            .repo
            .find_by_id(task_id, user_id)
            .await
            .map_err(AppError::from)?;

        task.map(Into::into).ok_or(AppError::NotFound)
    }

    pub async fn list_tasks(
        &self,
        user_id: Uuid,
        filters: TaskFilters,
        sort: TaskSort,
        page: i64,
        per_page: i64,
    ) -> Result<TaskListResponse, AppError> {
        let page = page.max(1);
        let per_page = per_page.clamp(1, MAX_PAGE_SIZE);

        let (tasks, total) = self
            .repo
            .find_all(user_id, filters, sort, page, per_page)
            .await
            .map_err(AppError::from)?;

        let data = tasks.into_iter().map(TaskResponse::from).collect();
        let meta = PaginationMeta::new(total, page, per_page);

        Ok(TaskListResponse { data, meta })
    }

    pub async fn update_task(
        &self,
        task_id: Uuid,
        user_id: Uuid,
        request: UpdateTaskRequest,
    ) -> Result<TaskResponse, AppError> {
        request
            .validate()
            .map_err(|err| AppError::Validation(err.to_string()))?;

        let task = self
            .repo
            .update(task_id, user_id, request)
            .await
            .map_err(AppError::from)?;

        Ok(task.into())
    }

    pub async fn delete_task(&self, task_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let affected = self
            .repo
            .soft_delete(task_id, user_id)
            .await
            .map_err(AppError::from)?;

        if affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}

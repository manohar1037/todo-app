use std::sync::Arc;

use bson::DateTime;
use bson::oid::ObjectId;
use mongodb::bson;
use serde::{Deserialize, Serialize};

use crate::{constant::MONGODB_TASKS_LOG_COLLECTION, error::AppError, state::AppState};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Pending,
    Progress,
    Rejected,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime>,
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime>,
    pub updated_by: Option<String>,
    pub remarks: Option<String>,
    pub active: bool,
}

impl Task {
    pub fn new(title: &str, username: &str) -> Self {
        let now = DateTime::now();
        Self {
            id: ObjectId::new(),
            title:title.to_string(),
            status: Status::Pending,
            created_at: Some(now),
            updated_at: Some(now),
            created_by: username.to_string(),
            updated_by: None,
            remarks: None,
            active: true,
        }
    }
}
#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub status: Status,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskLog {
    pub task_id: ObjectId,
    pub event: String, // e.g., "created", "status_changed", "updated", "deleted"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Status>,
    pub title: String,
    pub by: String, // username from basic auth
    pub at: DateTime,
}

impl TaskLog {
    pub fn new(
        task_id: ObjectId,
        event: &str,
        from: Option<Status>,
        to: Option<Status>,
        title:&str,
        by: &str,
        at: DateTime,
    ) -> Self {
        Self {
            task_id,
            event:event.to_string(),
            from,
            to,
            title:title.to_string(),
            by:by.to_string(),
            at,
        }
    }

    pub async fn create_task_log(&self, state: Arc<AppState>) -> Result<(), AppError> {
        let database = state.db();
        let task_logs_collection = database
            .collection::<TaskLog>(MONGODB_TASKS_LOG_COLLECTION);
        task_logs_collection.insert_one(self).await?;
        Ok(())
    }
}

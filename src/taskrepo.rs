use futures::TryStreamExt;
use mongodb::bson::{DateTime, Document, doc, oid::ObjectId, to_bson};
use std::sync::Arc;

use crate::{
    constant::MONGODB_TASKS_COLLECTION,
    error::AppError,
    model::{CreateTask, Task, TaskLog, UpdateTask},
    state::AppState,
};

pub async fn fetch_all_tasks(state: Arc<AppState>) -> Result<Vec<Task>, AppError> {
    let database = state.db();
    let task_collection = database.collection::<Task>(MONGODB_TASKS_COLLECTION);
    let cursor = task_collection
        .find(doc! {"active":true})
        .await
        .map_err(AppError::from)?;
    let tasks: Vec<Task> = cursor
        .try_collect::<Vec<_>>()
        .await
        .map_err(AppError::from)?;
    Ok(tasks)
}

pub async fn fetch_task(state: Arc<AppState>, id: &str) -> Result<Option<Task>, AppError> {
    let database = state.db();
    let task_collection = database.collection::<Task>(MONGODB_TASKS_COLLECTION);
    let oid = ObjectId::parse_str(id).map_err(|_| AppError::BadRequest("Invalid id".into()))?;
    let filter = doc! {"_id": oid};
    let task = task_collection.find_one(filter).await?;
    Ok(task)
}

pub async fn create_task_service(
    state: Arc<AppState>,
    username: String,
    payload: CreateTask,
) -> Result<Task, AppError> {
    let database = state.db();
    let task_collection = database.collection::<Task>(MONGODB_TASKS_COLLECTION);
    let now = DateTime::now();
    let task = Task::new(&payload.title, &username);
    task_collection.insert_one(&task).await?;

    let log = TaskLog::new(
        task.id,
        "created",
        None,
        Some(task.status.clone()),
        &task.title,
        &username,
        now,
    );
    log.create_task_log(state).await?;
    Ok(task)
}

pub async fn update_task_service(
    state: Arc<AppState>,
    username: String,
    id: String,
    payload: UpdateTask,
) -> Result<Option<Task>, AppError> {
    let database = state.db();
    let task_collection = database.collection::<Task>(MONGODB_TASKS_COLLECTION);
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid id".into()))?;

    let task = task_collection
        .find_one(doc! { "_id": oid })
        .await
        .map_err(AppError::from)?;

    let existing = match task {
        Some(t) => t,
        None => return Err(AppError::TaskNotFound(id)),
    };

    let remarks = match payload.remarks {
        Some(remarks) => remarks,
        None => return Err(AppError::NoRemarksFound),
    };

    let mut set_doc = Document::new();
    set_doc.insert("title", &existing.title);

    if existing.status == payload.status {
        return Err(AppError::SameStatus);
    }

    set_doc.insert(
        "status",
        to_bson(&payload.status).map_err(|_| AppError::BsonConversionError)?,
    );
    let now = DateTime::now();
    set_doc.insert("updated_at", now);
    set_doc.insert("updated_by", &username);
    set_doc.insert("remarks", remarks);
    
    let update_doc = doc! { "$set": set_doc };

    let res = task_collection
        .update_one(doc! { "_id": oid }, update_doc)
        .await
        .map_err(AppError::from)?;

    if res.matched_count == 0 {
        return Err(AppError::NotUpdated(
            "Error occurred while updating the task".into(),
        ));
    }

    let log = TaskLog::new(
        oid,
        "status changed",
        Some(existing.status),
        Some(payload.status),
        &existing.title,
        &username,
        now,
    );
    log.create_task_log(state).await?;
    // fetch updated document
    let task = task_collection
        .find_one(doc! { "_id": oid })
        .await
        .map_err(AppError::from)?;
    Ok(task)
}

pub async fn delete_task_serivce(
    state: Arc<AppState>,
    username: String,
    id: String,
) -> Result<(), AppError> {
    let database = state.db();
    let task_collection = database.collection::<Task>(MONGODB_TASKS_COLLECTION);
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid id".into()))?;
    let filter = doc! { "_id":oid};
    let task = task_collection.find_one(filter).await?;

    let exisiting_task = match task {
        Some(task) => task,
        None => return Err(AppError::TaskNotFound(id)),
    };

    let now = DateTime::now();
    let update_doc = doc! {
        "$set": {
            "active": false,
            "updated_at": now,
            "updated_by": &username
        }
    };

    let res = task_collection
        .update_one(doc! {"_id": oid}, update_doc)
        .await?;

    if res.matched_count == 0 {
        return Err(AppError::NotDeleted(
            "Error occurred while soft deleting the task".into(),
        ));
    }

    let log = TaskLog::new(
        exisiting_task.id,
        "soft deleted",
        Some(exisiting_task.status),
        None,
        &exisiting_task.title,
        &username,
        now,
    );
    log.create_task_log(state).await?;

    Ok(())
}

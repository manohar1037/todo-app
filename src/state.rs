use mongodb::Database;

use crate::{
    config::Config, constant::{MONGODB_DB, MONGODB_DB_URI}, db::connect_db, error::AppError
};

use std::env;

use dotenvy::dotenv;
#[derive(Debug, Clone)]
pub struct AppState {
    db: Database,
    config:Config
}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        dotenv().ok();
        let mongodb_uri =
            env::var(MONGODB_DB_URI).unwrap_or_else(|_| "mongodb://localhost:27017".into());
        let mongodb_db_name = env::var(MONGODB_DB).unwrap_or_else(|_| "task_db".into());
        let db = connect_db(mongodb_uri, mongodb_db_name).await?;

        let config = Config::new();

        Ok(Self { db ,config})
    }
    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn basic_user(&self)->String{
        self.config.basic_user.to_string()
    }
    pub fn basic_pass(&self)->String{
        self.config.basic_pass.to_string()
    }
    pub fn port(&self)->u16{
        self.config.port
    }
}

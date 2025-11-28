use mongodb::{Client, Database, options::ClientOptions};
pub async fn connect_db(mongodb_uri:String,mongodb_db_name:String) -> mongodb::error::Result<Database> {
        let client_options = ClientOptions::parse(mongodb_uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&mongodb_db_name);
        Ok(db)
}

mod services;
mod models;
mod repository;

use repository::mongodb_db_repo::MongoRepo;
use services::mqtt_service::connect_mqtt;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_db = MongoRepo::init().await;

    connect_mqtt(mongo_db).await
}
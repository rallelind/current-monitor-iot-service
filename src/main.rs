mod services;
mod models;
mod repository;

use services::mqtt_service::connect_mqtt;

#[tokio::main]
async fn main() {
    connect_mqtt()
}
mod services;
mod models;
mod repository;

use services::mqtt_service::connect_mqtt;

fn main() {
    connect_mqtt()
}
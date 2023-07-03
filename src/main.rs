mod services;
mod models;

use services::mqtt_service::connect_mqtt;

fn main() {
    connect_mqtt()
}
mod services;
use services::mqtt_service::connect_mqtt;

fn main() {
    connect_mqtt()
}
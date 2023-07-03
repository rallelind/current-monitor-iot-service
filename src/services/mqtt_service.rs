const DFLT_BROKER: &str = "broker.emqx.io";
const DFLT_CLIENT: &str = "rust_subscribe";
const DFLT_TOPICS: &str = &"rust/mqtt";
const DFLT_QOS: &i32 = &1;

use std::{env, process, thread, time::Duration};

extern crate paho_mqtt as mqtt;

use mqtt::{Client, CreateOptionsBuilder};

pub struct MqttService {
    pub mqtt_client: Client,
}

impl MqttService {
    pub fn new() -> Self {

        let client_builder_options = CreateOptionsBuilder::new()
            .server_uri(DFLT_BROKER.to_string())
            .client_id(DFLT_CLIENT.to_string())
            .finalize();

        let mqtt_client = Client::new(client_builder_options).unwrap_or_else(|err| {
            println!("Error creating client: {:?}", err);
            process::exit(1);
        });

        

        MqttService { mqtt_client }
    }

    fn subscribe_topics(&self) {
        if let Err(e) = self.mqtt_client.subscribe(DFLT_TOPICS, *DFLT_QOS) {
            println!("Error subscribes topics: {:?}", e);
            process::exit(1);
        }
    }

    fn reconnect(&self) -> bool {
        println!("Connection lost. Reconnecting!");
        for _ in 0..12 {
            thread::sleep(Duration::from_millis(5000));
            if self.mqtt_client.reconnect().is_ok() {
                println!("Reconnected!");
                return true;
            }
        }
        println!("Could not reconnect");
        false
    }

    pub fn disconnect(&self) {
        if self.mqtt_client.is_connected() {
            println!("Disconnecting mqtt client!");
            self.mqtt_client.unsubscribe(DFLT_TOPICS).unwrap();
            self.mqtt_client.disconnect(None).unwrap();
        }
    }
}

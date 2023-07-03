use mongodb::{Client, Collection};
use std::env::var;

use crate::models::current_model::CurrentMonitor;

pub struct MongoRepo {
    pub current_monitor_collection: Collection<CurrentMonitor>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let mongo_connection_string =
            var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

        let client = Client::with_uri_str(mongo_connection_string)
            .await
            .expect("error connection to client");

        let db = client.database("current_monitor");

        let current_monitor_collection: Collection<CurrentMonitor> =
            db.collection("current_monitor");

        MongoRepo {
            current_monitor_collection,
        }
    }
}

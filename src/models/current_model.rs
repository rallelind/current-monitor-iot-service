use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

use crate::repository::mongodb_db_repo::MongoRepo;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CurrentMonitor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "mongodb::bson::serde_helpers::bson_datetime_as_rfc3339_string")]
    pub time_stamp: DateTime,
    pub current_value: i32,
}

impl MongoRepo {

    pub async fn process_current_data(&self, current_data: CurrentMonitor) {
        self.current_monitor_collection.insert_one(current_data, None).await.unwrap();
    }

}
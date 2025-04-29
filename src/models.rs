use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlayerScore {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String, 
    pub score: i32,
}
#[derive(Deserialize, Debug)]
pub struct NewScore {
    pub name: String,
    pub score: u32,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct PlayerScore {
    pub id: String,
    pub name: String, 
    pub score: i32,
}
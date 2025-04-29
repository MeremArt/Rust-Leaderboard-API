
use super::models::PlayerScore; 
use mongodb::Collection;
use std::sync::Mutex;

pub struct AppState {
    pub leaderboard_collection: Collection<PlayerScore>,
}
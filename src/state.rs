
use super::models::PlayerScore; 
use std::sync::Mutex;

pub struct AppState {
    pub leaderboard: Mutex<Vec<PlayerScore>>,
}
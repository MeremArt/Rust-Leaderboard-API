
use super::models::PlayerScore; 
use mongodb::Collection;


pub struct AppState {
    pub leaderboard_collection: Collection<PlayerScore>,
}
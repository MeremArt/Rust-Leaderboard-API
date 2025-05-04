
use super::models::{PlayerScore, User}; 
use mongodb::Collection;


pub struct AppState {
    pub leaderboard_collection: Collection<PlayerScore>,
    pub users_collection: Collection<User>,
    pub jwt_secret: String,
}
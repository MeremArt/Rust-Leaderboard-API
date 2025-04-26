use actix_web::{get,post, delete,web,HttpResponse,Responder};

use uuid::Uuid;
use crate::{models::PlayerScore, state::AppState, error::ApiError};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct NewScore {
    pub name: String,
    pub score: u32,
}



#[post("/leaderboard")]
async fn add_score(
    data: web::Data<AppState>,
    param: web::Json<NewScore>,
) -> impl Responder {

    if param.name.trim().is_empty() {
        return Err(ApiError::BadRequest);
    }
    let new_player = PlayerScore {
        id: Uuid::new_v4().to_string(),
        name: param.name.clone(),
        score: param.score as i32,
    };

    let mut leaderboard = data.leaderboard.lock().unwrap();
    leaderboard.push(new_player);

    Ok(HttpResponse::Created().json(json!({ "message": "Score added!" })))

}
#[get("/leaderboard/top/{count}")]
async fn get_top(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> impl Responder {
    let count = path.into_inner();
    let leaderboard = match data.leaderboard.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(ApiError::InternalError),
    };
    

    let mut sorted = leaderboard.clone();
    sorted.sort_by(|a, b| b.score.cmp(&a.score));

    let top_players: Vec<_> = sorted.into_iter().take(count).collect();

    Ok(HttpResponse::Ok().json(top_players))
}

#[delete(
    "/leaderboard/{id}"
)]

async fn delete_score( data:web::Data<AppState>, path:web::Path<String>) -> impl Responder {
let mut leaderboard = data.leaderboard.lock().unwrap();
let id = path.into_inner();
leaderboard.retain(|player| player.id != id);
let original_len = leaderboard.len();
if leaderboard.len() == original_len {
    // No player was deleted
    return Err(ApiError::NotFound);
}

*leaderboard = leaderboard.iter().filter(|&x| x.id != id).cloned().collect();
Ok(HttpResponse::Ok().json(json!({ "message": "Player deleted successfully" })))


}
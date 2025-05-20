use actix_web::{get,post, delete,web,HttpResponse,Responder};
use crate::{ models::{NewScore,UserSignup}, state::AppState, error::ApiError ,services};


#[get("/leaderboard")]
async fn get_all(
    data: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let players = services::get_all_players(&data.leaderboard_collection).await?;
    Ok(HttpResponse::Ok().json(players))
}

#[post("/leaderboard")]
async fn add_score(
    data: web::Data<AppState>,
    param: web::Json<NewScore>,
) -> Result<impl Responder, crate::error::ApiError> {
    let new_player = services::add_player(&data.leaderboard_collection, param.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_player))
}
#[post("/signup")]
async fn signup(
    data: web::Data<AppState>,
    info: web::Json<UserSignup>,
) ->  Result<impl Responder, crate::error::ApiError> {
    services::signup(&data.users_collection, info.into_inner(), &data.jwt_secret).await?;
    Ok(HttpResponse::Created().json(serde_json::json!({ "message": "User created" })))
}

#[delete("/leaderboard/{id}")]
async fn delete_score(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, crate::error::ApiError> {
    services::delete_player(&data.leaderboard_collection, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Player deleted successfully." })))
}

#[get("/leaderboard/top/{count}")]
async fn get_top(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, ApiError> {
    let count = path.into_inner();
    let top_players = services::get_top_players(&data.leaderboard_collection, count).await?;
    Ok(HttpResponse::Ok().json(top_players))
}
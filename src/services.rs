
use crate::models::{NewScore, PlayerScore, UserSignup, User, UserLogin};
use crate::auth::{hash_password, verify_password, create_jwt};
use crate::error::ApiError;
use mongodb::bson::doc;

use mongodb::bson::oid::ObjectId;
use futures::stream::StreamExt;
use mongodb::Collection;
use mongodb::options::FindOptions;

pub async fn add_player(
    collection: &mongodb::Collection<PlayerScore>,
    new_score: NewScore,
) -> Result<PlayerScore, ApiError> {
    if new_score.name.trim().is_empty() {
        return Err(ApiError::BadRequest);
    }

    let new_player = PlayerScore {
        id: None,
        name: new_score.name,
        score: new_score.score as i32,
    };

    let insert_result = collection.insert_one(new_player.clone(), None).await
        .map_err(|_| ApiError::InternalError)?;

    Ok(PlayerScore {
        id: Some(insert_result.inserted_id.as_object_id().unwrap()),
        name: new_player.name,
        score: new_player.score,
    })
}

pub async fn get_all_players(
    collection: &Collection<PlayerScore>,
) -> Result<Vec<PlayerScore>, ApiError> {
    let mut cursor = collection.find(None, None).await
        .map_err(|_| ApiError::InternalError)?;

    let mut players = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(player) => players.push(player),
            Err(_) => return Err(ApiError::InternalError),
        }
    }

    Ok(players)
}

pub async fn delete_player(
    collection: &mongodb::Collection<PlayerScore>,
    id: String,
) -> Result<(), ApiError> {
    let obj_id = ObjectId::parse_str(&id).map_err(|_| ApiError::BadRequest)?;

    let delete_result = collection.delete_one(doc! { "_id": obj_id }, None).await
        .map_err(|_| ApiError::InternalError)?;

    if delete_result.deleted_count == 1 {
        Ok(())
    } else {
        Err(ApiError::NotFound)
    }
}
pub async fn get_top_players(
    collection: &Collection<PlayerScore>,
    count: usize,
) -> Result<Vec<PlayerScore>, ApiError> {
    let options = FindOptions::builder()
        .sort(doc! { "score": -1 }) // 👈 sort descending
        .limit(count as i64)        // 👈 take top `count`
        .build();

    let mut cursor = collection.find(None, options).await
        .map_err(|_| ApiError::InternalError)?;

    let mut players = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(player) => players.push(player),
            Err(_) => return Err(ApiError::InternalError),
        }
    }

    Ok(players)
}

pub async fn signup(
    users:&Collection<User>,
    signup: UserSignup,
    jwt_secret: &str,
) -> Result<(), ApiError> {

    if signup.username.trim().is_empty() || signup.password.len() < 6 {
        return Err(ApiError::BadRequest);
    }

    let exists = users
    .find_one(doc! { "username": &signup.username }, None)
    .await
    .map_err(|_| ApiError::InternalError)?;
      
if exists.is_some() {
    return Err(ApiError::BadRequest);
}

let password_hash = hash_password(&signup.password, jwt_secret)?;


let user = User {
    username: signup.username,
    password_hash,
    role: "user".to_string(),
};

users.insert_one(user,None).await .map_err(|_| ApiError::InternalError)?;

Ok(())
}

pub async fn login (    users: &Collection<User>,
    creds: UserLogin,
    jwt_secret: &str,)-> Result<String,ApiError>{
        let user = users
        .find_one(doc! { "username": &creds.username }, None)
        .await
        .map_err(|_| ApiError::InternalError)?
        .ok_or(ApiError::BadRequest)?;

    let is_valid = verify_password(&user.password_hash, &creds.password, jwt_secret)?;

    if !is_valid {
        return Err(ApiError::BadRequest);
    }

    create_jwt(&user.username, jwt_secret)
    }
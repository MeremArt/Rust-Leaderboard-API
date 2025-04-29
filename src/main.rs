
use actix_web::{get, web, App, HttpServer};
use monogodb::{Client, options::ClientOptions};
use std::sync::Mutex;
use crate::state::AppState;
use crate::routes::{add_score, get_top, delete_score};
use std::env;
use dotenvy::dotenv;

mod state;
mod models;
mod routes;
mod error;


#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db_name = env::var("MONGO_DB_NAME").unwrap_or("leaderboard_db".to_string());


    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    let collection = db.collection::<models::PlayerScore>("leaderboard");

    let app_data = web::Data::new(AppState {
    
        leaderboard_collection: Mutex::new(vec![]),
    });
    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".into())    
    .parse::<u16>()                       
    .expect("PORT must be a valid u16");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
           .service(index)
            .service(add_score)
            .service(get_top)
            .service(delete_score)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
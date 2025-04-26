
use actix_web::{get, web, App, HttpServer};

use std::sync::Mutex;

mod state;
mod models;
mod routes;
mod error;


use crate::state::AppState;
use crate::routes::{add_score, get_top, delete_score};
#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        leaderboard: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
           
            .service(add_score)
            .service(get_top)
            .service(delete_score)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
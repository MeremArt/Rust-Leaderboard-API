
use actix_web::{get, web, App, HttpServer};
use mongodb::{Client, options::ClientOptions};
use actix_cors::Cors;
use crate::state::AppState;
use crate::routes::{add_score, get_top, delete_score, get_all};
use std::env;
use dotenvy::dotenv;

use rustls::{Certificate, PrivateKey, ServerConfig};
use std::fs::File;
use std::io::BufReader;


mod state;
mod models;
mod routes;
mod error;
mod services;
mod auth;



#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let db_name = env::var("MONGO_DB_NAME").unwrap_or("leaderboard_db".to_string());
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);
    let users_collection = db.collection::<models::User>("users");
    let collection = db.collection::<models::PlayerScore>("leaderboard");
    let app_data = web::Data::new(AppState {
        leaderboard_collection: collection,
        users_collection,
        jwt_secret,
    });
    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".into())    
    .parse::<u16>()                       
    .expect("PORT must be a valid u16");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Cors::permissive())
           .service(index)
            .service(add_score)
            .service(get_top)
            .service(get_all)
            .service(delete_score)
    })
    .bind_rustls(("127.0.0.1", port), load_rustls_config())?
    .run()
    .await
}

fn load_rustls_config() -> rustls::ServerConfig {
    use rustls::{Certificate, PrivateKey, ServerConfig};
    use std::fs::File;
    use std::io::BufReader;

    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file).unwrap();
    let key = PrivateKey(keys.remove(0));

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .unwrap()
}

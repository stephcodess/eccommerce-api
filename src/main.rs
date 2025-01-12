use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod handlers;
mod models;
mod services;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(handlers::user_handler::create_user_endpoint)
            .service(handlers::user_handler::fetch_all_users)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

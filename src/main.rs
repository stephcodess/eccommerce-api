use actix_web::{web::ServiceConfig, App};
use shuttle_actix_web::ShuttleActixWeb;
use dotenvy::dotenv;

mod handlers;
mod models;
mod services;
mod schema;
mod utils;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv().ok();

    // Actix Web configuration function
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(handlers::user_handler::auth::create_user_endpoint);
        cfg.service(handlers::user_handler::users::fetch_all_users);
        cfg.service(handlers::user_handler::users::get_user_by_id);
    };

    Ok(config.into())
}

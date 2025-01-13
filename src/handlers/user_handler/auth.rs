use actix_web::{post, web, HttpResponse, Responder};
use auth_flow::establish_connection;
use crate::services::auth::create_user::create_new_user;
use crate::models::user_model::NewUser;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub user_password: String,
    pub phone_number: Option<String>,
}

#[post("/signup")]
async fn create_user_endpoint(user_data: web::Json<CreateUserRequest>) -> impl Responder {
    let conn = &mut establish_connection();

    let new_user = NewUser {
        first_name: &user_data.first_name,
        last_name: &user_data.last_name,
        username: &user_data.username,
        email: &user_data.email,
        user_password: &user_data.user_password,
        phone_number: user_data.phone_number.as_deref(),
        date_created: chrono::Utc::now().naive_utc(),
    };

    match create_new_user(conn, new_user) {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => {
            eprintln!("Failed to create user: {}", err);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}


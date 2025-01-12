use actix_web::{web, get, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::user_model::Users;
use crate::schema::users::dsl::*;
use auth_flow::establish_connection;

#[get("/users")]
async fn fetch_all_users() -> impl Responder {
    let conn = &mut establish_connection();

    match users.limit(10)
        .load::<Users>(conn)
    {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(err) => {
            eprintln!("Failed to fetch users: {}", err);
            HttpResponse::InternalServerError().body("Failed to fetch users")
        }
    }
}


#[get("/user/{id}")]
async fn get_user_by_id(user_id: web::Path<i32>) -> impl Responder {
    let conn = &mut establish_connection();

    match users.filter(id.eq(user_id.into_inner()))
        .first::<Users>(conn)
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("Failed to fetch user by id {:?}: {}", id, err);
            HttpResponse::InternalServerError().body("Failed to fetch user")
        }
    }
}

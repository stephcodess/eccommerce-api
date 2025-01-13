use diesel::prelude::*;
use diesel::Queryable;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub user_password: String,
    pub phone_number: Option<String>,
    pub date_created: Option<NaiveDateTime>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub username: &'a str,
    pub email: &'a str,
    pub user_password: &'a str,
    pub phone_number: Option<&'a str>,
    pub date_created: NaiveDateTime,
}


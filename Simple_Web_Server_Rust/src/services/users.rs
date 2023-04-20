use Simple_Web_Server_Rust::{establish_connection, schema::users};
use diesel::{Connection, PgConnection, RunQueryDsl};
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;
use crate::models::models::UserInputUser;
use crate::models::models::User;

pub fn create_user_input(user_info: &UserInputUser) -> Value {
    let mut connection = establish_connection();
    let new_user = User {
        email: &user_info.email,
        password: &user_info.password,
    };
    let result = diesel::insert_into(users::table).values(&new_user).execute(&mut connection);
    match result {
        Ok(user) => json!({
            "status": "success",
            "data": user
        }),
        Err(e) => json!({
            "status": "error",
            "message": e.to_string()
        }),
    }
}
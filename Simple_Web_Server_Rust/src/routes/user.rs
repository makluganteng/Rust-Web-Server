use rocket::serde::json::{Json, Value};

use crate::models::models::{UserInputUser};
use crate::services::users::{self, create_user_input};

#[post("/users/create-user", format = "json", data = "<user_info>")]
pub fn create_user(user_info: Json<UserInputUser>) -> Value {
    create_user_input(&user_info)
}
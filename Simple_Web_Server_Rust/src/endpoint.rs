//Place to declare endpoints
use serde::{Deserialize};
use rocket::serde::json::Json;
#[derive(Debug, Deserialize)]
pub struct Data {
    name: String,
    number: i32,
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/name")]
pub fn index2() -> &'static str {
    "Hello, Valentino!"
}

#[post("/test", data = "<post>", format = "json")]
pub fn test(post: Option<Json<Data>>) -> String {
    match post {
        Some(json_data) => {
            let data: Data = json_data.0;
            format!("Received data: {:?}", data)
        },
        None => "No data received".to_string(),
    }
}

// #[post("/test2", data = "<post>")]
// pub fn test2(post: Json<Post>) -> Json<Post> {
//     post
// }
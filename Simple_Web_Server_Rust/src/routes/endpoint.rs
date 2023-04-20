//Place to declare endpoints
use serde::{Deserialize};
use rocket::{serde::json::Json, http::Status};
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

#[get("/id/<code>")]
pub fn forced_error(code: u16) -> Status {
    Status::new(code)
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

#[catch(404)]
pub fn not_found(req: &rocket::Request<'_>) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
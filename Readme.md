# Create a simple Rust Web Server

This is a simple rust web server where it does CRUD functions. We are building this using Rocket Web Framework.

Example of using GET Method 

```rust
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
```

Example of using POST Method 

```rust
use rocket::serde::json::Json;

pub struct Data {
    name: String,
    number: i32,
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
```

Rest still WIP
#[macro_use] extern crate rocket;
mod routes {
    pub mod endpoint;
    pub mod user;
}
mod models {
    pub mod models;
}

mod services {
    pub mod users;
}

use routes::endpoint;
use routes::user::create_user;
use endpoint::index;
use endpoint::index2;
use endpoint::test;
use endpoint::forced_error;
#[launch] // declare multiple endpoints in route
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,index2,test,forced_error,create_user])
    .register("/", catchers![endpoint::not_found])
}


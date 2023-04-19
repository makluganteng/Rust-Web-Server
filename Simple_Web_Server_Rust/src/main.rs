#[macro_use] extern crate rocket;
mod endpoint;
use endpoint::index;
use endpoint::index2;
use endpoint::test;
#[launch] // declare multiple endpoints in route
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,index2,test])
}


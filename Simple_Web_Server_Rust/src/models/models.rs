use Simple_Web_Server_Rust::schema::users;
use diesel::{prelude::*};
use rocket::serde::{Serialize, Deserialize};


#[derive(Insertable, Serialize, AsChangeset)]
#[diesel(table_name = users)]
pub struct User<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Deserialize)]
pub struct UserInputUser {
    pub email: String,
    pub password: String,
}
 
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::{User, NewUser};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(
    conn: &PgConnection, 
    first_name: &'a str, 
    last_name: &'a str,
    age: &'a i32,
    email: &'a str
) -> User {
    use schema::users;

    let new_post = NewUser {
        first_name: first_name,
        last_name: last_name,
        age: age,
        email: email,
    };

    diesel::insert_into(users::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
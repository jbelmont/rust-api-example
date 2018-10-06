extern crate rust_api_example;
extern crate diesel;

use self::diesel::prelude::*;
use self::rust_api_example::*;
use rust_api_example::schema::users::dsl::*;
use std::env::args;


fn main() {
    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(users.filter(first_name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting user");

    println!("Deleted {} user", num_deleted);
}
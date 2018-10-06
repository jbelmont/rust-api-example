extern crate diesel;
extern crate rust_api_example;

use rust_api_example::*;
use self::models::*;
use diesel::prelude::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.first_name);
    }
}
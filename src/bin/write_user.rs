extern crate rust_api_example;
extern crate diesel;

use self::rust_api_example::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your first name to be?");
    let mut first_name = String::new();
    stdin().read_line(&mut first_name).unwrap();
    let first_name = &first_name[..(first_name.len() - 1)]; // Drop the newline character
    
    println!("What would you like your last name to be?");
    let mut last_name = String::new();
    stdin().read_line(&mut last_name).unwrap();
    let last_name = &last_name[..(last_name.len() - 1)]; // Drop the newline character

    let age = 5;

    println!("What would you like your email to be?");
    let mut email = String::new();
    stdin().read_line(&mut email).unwrap();
    let email = &email[..(email.len() - 1)]; // Drop the newline character

    let user = create_user(&connection, first_name, last_name, &age, email);
    println!("\nSaved user {} with id {}", first_name, user.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
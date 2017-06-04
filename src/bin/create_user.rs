extern crate schani_userinfo;
extern crate diesel;

use schani_userinfo::db::establish_connection;
use schani_userinfo::users::create_user;
use std::io::stdin;

fn main() {
    let connection = establish_connection();

    println!("What would you like your username and password to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)]; // Drop the newline character

    let user = create_user(&connection, &username.trim(), &password.trim());
    println!("\nSaved user {} with id {}", username, user.id);
}

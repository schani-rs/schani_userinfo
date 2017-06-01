#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate schani_userinfo;
extern crate rocket;

use std::result::Result;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status::Custom;
use schani_userinfo::{establish_connection, verify_password};

#[derive(Debug, FromForm)]
struct Credentials {
    password: String,
}

#[post("/user/<username>/authenticate", data = "<credentials>")]
fn authenticate(username: String, credentials: Form<Credentials>) -> Result<(), Custom<&'static str>> {
    let conn = establish_connection();
    let creds = credentials.get();

    if verify_password(&conn, &username, &creds.password) {
        Ok(())
    } else {
        Err(Custom(Status::Unauthorized, "invalid username/password"))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![authenticate]).launch();
}

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate schani_userinfo;
extern crate rocket;
extern crate rocket_contrib;

use std::result::Result;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status::Custom;
use rocket_contrib::JSON;
use schani_userinfo::*;
use schani_userinfo::models::Setting;

#[derive(Debug, FromForm)]
struct Credentials {
    password: String,
}

#[post("/user/<username>/authenticate", data = "<credentials>")]
fn authenticate(username: String,
                credentials: Form<Credentials>)
                -> Result<(), Custom<&'static str>> {
    let conn = establish_connection();
    let creds = credentials.get();

    if verify_password(&conn, &username, &creds.password) {
        Ok(())
    } else {
        Err(Custom(Status::Unauthorized, "invalid username/password"))
    }
}

#[derive(Debug, FromForm)]
struct SettingData {
    key: String,
    value: String,
}

#[put("/user/<user_id>/setting", data = "<setting_data>")]
fn setting_save(user_id: i32, setting_data: Form<SettingData>) -> Result<(), Custom<String>> {
    let conn = establish_connection();
    let setting = setting_data.get();

    match save_setting(&conn, user_id, &setting.key, &setting.value) {
        Ok(_) => Ok(()),
        Err(err) => Err(Custom(Status::InternalServerError, err)),
    }
}

#[get("/user/<user_id>/setting/<key>")]
fn setting_get(user_id: i32, key: String) -> Result<JSON<Setting>, Custom<String>> {
    let conn = establish_connection();

    match get_setting(&conn, user_id, &key) {
        Ok(setting) => Ok(JSON(setting)),
        Err(err) => Err(Custom(Status::InternalServerError, err)),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![authenticate, setting_save, setting_get])
        .launch();
}

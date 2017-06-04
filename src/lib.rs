#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate pwhash;
#[macro_use]
extern crate serde_derive;

pub mod auth;
pub mod db;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use pwhash::bcrypt;
use self::models::{User, NewUser, Setting, NewSetting};

pub fn get_users<'a>(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users.load::<User>(conn).expect("Error loading users")
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use schema::users;

    let password = bcrypt::hash(password).unwrap();
    let new_user = NewUser {
        username: username,
        password: &password,
    };

    diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn save_setting<'a>(conn: &PgConnection,
                        user_id: i32,
                        key: &'a str,
                        value: &'a str)
                        -> Result<Setting, String> {
    use schema::settings;

    let new_setting = NewSetting {
        user_id: user_id,
        key: key,
        value: value,
    };

    // TODO: overwrite existing
    diesel::insert(&new_setting)
        .into(settings::table)
        .get_result(conn)
        .map_err(|err| err.to_string())
}

/// TODO: find a way to remove pararmeter/query ambiguity
pub fn get_setting<'a>(conn: &PgConnection,
                       q_user_id: i32,
                       q_key: &'a str)
                       -> Result<Setting, String> {
    use schema::settings::dsl::*;

    settings
        .filter(user_id.eq(q_user_id))
        .filter(key.eq(q_key))
        .limit(1)
        .get_result(conn)
        .map_err(|err| err.to_string())
}

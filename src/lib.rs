#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{User, NewUser, Setting, NewSetting};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn hash_password<'a, S>(password: S) -> String
    where S: Into<String>
{
    let p: String = password.into();
    p.to_owned()
}

pub fn get_users<'a>(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users.load::<User>(conn).expect("Error loading users")
}

pub fn verify_password<'a>(conn: &PgConnection, user: &String, pwd: &String) -> bool {
    use schema::users::dsl::*;

    match users
              .filter(username.eq(user))
              .filter(password.eq(hash_password(pwd.to_owned())))
              .limit(1)
              .get_result::<User>(conn) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use schema::users;

    let password = hash_password(password);
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

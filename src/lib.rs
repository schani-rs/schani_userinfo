#![feature(custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate pwhash;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use pwhash::bcrypt;
use std::env;
use self::models::{User, NewUser, Setting, NewSetting};

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_users<'a>(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users.load::<User>(conn).expect("Error loading users")
}

pub fn verify_password<'a>(conn: &PgConnection, user: &String, pwd: &String) -> bool {
    use schema::users::dsl::*;

    let user = match users
              .filter(username.eq(user))
              .limit(1)
              .get_result::<User>(conn) {
        Ok(user) => user,
        Err(_) => return false,
    };

    bcrypt::verify(pwd, &user.password)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_password_with_nonexistent_user() {
        let conn = establish_connection();

        assert!(verify_password(&conn, &"ferdinand".to_string(), &"123456".to_string()));
    }

    #[test]
    fn verify_password_with_correct_password() {
        let conn = establish_connection();

        create_user(&conn, "ferdinand", "123456");

        assert!(verify_password(&conn, &"ferdinand".to_string(), &"123456".to_string()));
    }

    #[test]
    fn verify_password_with_incorrect_password() {
        let conn = establish_connection();

        create_user(&conn, "ferdinand", "123456");

        assert_eq!(false, verify_password(&conn, &"ferdinand".to_string(), &"wrong".to_string()));
    }
}

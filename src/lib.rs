#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
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

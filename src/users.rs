use diesel::insert;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use pwhash::bcrypt;
use super::models::{User, NewUser};

pub fn get_users<'a>(conn: &PgConnection) -> Vec<User> {
    use super::schema::users::dsl::*;

    users.load::<User>(conn).expect("Error loading users")
}

pub fn create_user<'a>(conn: &PgConnection, username: &'a str, password: &'a str) -> User {
    use super::schema::users;

    let password = bcrypt::hash(password).unwrap();
    let new_user = NewUser {
        username: username,
        password: &password,
    };

    insert(&new_user)
        .into(users::table)
        .get_result(conn)
        .expect("Error saving new user")
}

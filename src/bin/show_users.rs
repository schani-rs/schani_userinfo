extern crate schani_userinfo;
extern crate diesel;

use schani_userinfo::db::establish_connection;
use schani_userinfo::users::get_users;

fn main() {
    let connection = establish_connection();
    let users = get_users(&connection);

    println!("Displaying {} users", users.len());
    for user in users {
        println!("{}: {}", user.id, user.username);
    }
}

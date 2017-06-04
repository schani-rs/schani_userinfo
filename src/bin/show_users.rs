extern crate schani_userinfo;
extern crate diesel;

use self::schani_userinfo::get_users;
use schani_userinfo::db::establish_connection;

fn main() {
    let connection = establish_connection();
    let users = get_users(&connection);

    println!("Displaying {} users", users.len());
    for user in users {
        println!("{}: {}", user.id, user.username);
    }
}

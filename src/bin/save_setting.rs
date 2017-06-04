extern crate schani_userinfo;
extern crate diesel;

use std::env::args;

use self::schani_userinfo::save_setting;
use schani_userinfo::db::establish_connection;

fn main() {
    let connection = establish_connection();

    let user_id = args().nth(1).expect("save_setting requires a user_id")
        .parse::<i32>().expect("Invalid user_id");
    let key = args().nth(2).expect("save_setting requires a key")
        .parse::<String>().expect("Invalid key");
    let value = args().nth(3).expect("save_setting requires a value")
        .parse::<String>().expect("Invalid value");
    let setting = save_setting(&connection, user_id, &key, &value).expect("could not save setting");

    println!("Saved setting {} with value {} for user ", setting.key, setting.value);
}
use diesel::insert;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::models::{Setting, NewSetting};

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
    insert(&new_setting)
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
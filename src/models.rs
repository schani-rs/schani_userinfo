use super::schema::{users, settings};

#[derive(Identifiable, Queryable, Associations)]
#[has_many(settings)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(User)]
pub struct Setting {
    pub id: i32,
    pub user_id: i32,
    pub key: String,
    pub value: String,
}

#[derive(Insertable)]
#[table_name="settings"]
pub struct NewSetting<'a> {
    pub user_id: i32,
    pub key: &'a str,
    pub value: &'a str,
}

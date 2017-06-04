use diesel::prelude::*;
use diesel::pg::PgConnection;
use pwhash::bcrypt;
use super::schema::users::dsl::*;
use super::models::User;

pub fn verify_password<'a>(conn: &PgConnection, user: &String, pwd: &String) -> bool {
    let user = match users
              .filter(username.eq(user))
              .limit(1)
              .get_result::<User>(conn) {
        Ok(user) => user,
        Err(_) => return false,
    };

    bcrypt::verify(pwd, &user.password)
}

#[cfg(test)]
mod tests {
    use super::verify_password;
    use super::super::db::establish_connection;
    use super::super::users::create_user;

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

        assert_eq!(false,
                   verify_password(&conn, &"ferdinand".to_string(), &"wrong".to_string()));
    }
}

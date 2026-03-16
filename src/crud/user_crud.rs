use crate::models::user_models::User;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// User CRUD
pub fn find_user_by_email(conn: &mut SqliteConnection, user_email: &str) -> QueryResult<User> {
    users.filter(email.eq(user_email)).first::<User>(conn)
}

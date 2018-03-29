use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
}

impl User {
    pub fn find_with_username(name: &str, conn: &PgConnection) -> Option<User> {
        use schema::users::dsl::*;

        users.filter(username.eq(name)).get_result(conn).ok()
    }
}

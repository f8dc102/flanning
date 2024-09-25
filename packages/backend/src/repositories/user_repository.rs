// src/repositories/user_repository.rs
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub fn create_user(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users).values(new_user).get_result(conn)
    }

    pub fn find_user_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        users.filter(email.eq(user_email)).first::<User>(conn)
    }

    pub fn find_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        users.filter(id.eq(user_id)).first::<User>(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }
}

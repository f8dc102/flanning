// src/services/auth_service.rs
use crate::models::user::{NewUser, User};
use crate::repositories::database::DbConn;
use crate::repositories::user_repository::UserRepository;
use bcrypt::{hash, verify};
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub fn register_user(
        conn: &mut DbConn,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User, String> {
        let password_hash = hash(password, 4).map_err(|_| "Hashing failed")?;
        let new_user = NewUser {
            id: Uuid::new_v4(),
            username: username.to_owned(),
            email: email.to_owned(),
            password_hash,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
        };
        UserRepository::create_user(conn, &new_user).map_err(|e| e.to_string())
    }

    pub fn login_user(
        conn: &mut DbConn,
        email_input: &str,
        password: &str,
    ) -> Result<User, String> {
        let user =
            UserRepository::find_user_by_email(conn, email_input).map_err(|_| "User not found")?;
        if verify(password, &user.password_hash).map_err(|_| "Verification failed")? {
            Ok(user)
        } else {
            Err("Invalid credentials".into())
        }
    }

    pub fn unregister_user(conn: &mut DbConn, user_id: Uuid) -> Result<usize, String> {
        UserRepository::delete_user(conn, user_id).map_err(|e| e.to_string())
    }

    pub fn login_user_by_id(
        conn: &mut DbConn,
        user_id: Uuid,
        password: &str,
    ) -> Result<User, String> {
        let user = UserRepository::find_user_by_id(conn, user_id).map_err(|_| "User not found")?;
        if verify(password, &user.password_hash).map_err(|_| "Verification failed")? {
            Ok(user)
        } else {
            Err("Invalid credentials".into())
        }
    }
}

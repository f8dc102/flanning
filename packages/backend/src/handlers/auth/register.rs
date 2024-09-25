// src/handlers/auth/register.rs
use crate::repositories::database::DbPool;
use crate::services::auth_service::AuthService;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(data: web::Json<RegisterInfo>, db_pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Couldn't get DB connection from pool");
    match AuthService::register_user(&mut conn, &data.username, &data.email, &data.password) {
        Ok(user) => HttpResponse::Ok().json(serde_json::json!({
            "id": user.id,
            "username": user.username,
            "email": user.email,
        })),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

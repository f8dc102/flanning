// src/handlers/auth/login.rs
use crate::repositories::database::DbPool;
use crate::services::auth_service::AuthService;
use crate::utils::auth::create_token;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(data: web::Json<LoginInfo>, db_pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = db_pool.get().expect("Couldn't get DB connection from pool");
    match AuthService::login_user(&mut conn, &data.email, &data.password) {
        Ok(user) => match create_token(user.id) {
            Ok(token) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
            Err(_) => HttpResponse::InternalServerError().body("Token creation failed"),
        },
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}

// src/handlers/profile/view_profile.rs
use crate::models::user::User;
use crate::services::user_service::UserService;
use crate::utils::auth::AuthGuard;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/profile")]
pub async fn view_profile(pool: web::Data<PgPool>, auth: AuthGuard) -> impl Responder {
    match UserService::get_user_by_id(&pool, auth.user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

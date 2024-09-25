// src/handlers/profile/edit_profile.rs
use crate::services::user_service::UserService;
use crate::utils::auth::AuthGuard;
use actix_web::{put, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct EditProfileRequest {
    name: Option<String>,
    bio: Option<String>,
    profile_image: Option<String>,
}

#[put("/profile")]
pub async fn edit_profile(
    pool: web::Data<PgPool>,
    auth: AuthGuard,
    form: web::Json<EditProfileRequest>,
) -> impl Responder {
    match UserService::update_user_profile(&pool, auth.user_id, form.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

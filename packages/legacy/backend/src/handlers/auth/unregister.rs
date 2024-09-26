// src/handlers/auth/unregister.rs
use crate::repositories::database::DbPool;
use crate::services::auth_service::AuthService;
use crate::utils::auth::Claims;
use actix_web::{delete, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UnregisterInfo {
    pub password: String,
}

#[delete("/unregister")]
pub async fn unregister(
    data: web::Json<UnregisterInfo>,
    db_pool: web::Data<DbPool>,
    req: HttpRequest,
) -> impl Responder {
    if let Some(claims) = req.extensions_mut().get::<Claims>() {
        let mut conn = db_pool.get().expect("Couldn't get DB connection from pool");
        // Verify password before deletion
        match AuthService::login_user_by_id(&mut conn, claims.sub, &data.password) {
            Ok(_) => match AuthService::unregister_user(&mut conn, claims.sub) {
                Ok(_) => HttpResponse::Ok().body("User unregistered successfully"),
                Err(err) => HttpResponse::InternalServerError().body(err),
            },
            Err(err) => HttpResponse::Unauthorized().body(err),
        }
    } else {
        HttpResponse::Unauthorized().body("Authorization token missing or invalid")
    }
}

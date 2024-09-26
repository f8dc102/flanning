// src/handlers/auth/logout.rs
use crate::utils::auth::Claims;
use actix_web::{post, HttpMessage, HttpRequest, HttpResponse, Responder};

#[post("/logout")]
pub async fn logout(req: HttpRequest) -> impl Responder {
    if let Some(claims) = req.extensions_mut().get::<Claims>() {
        // Invalidate token logic if necessary
        HttpResponse::Ok().body(format!("User {} logged out successfully", claims.sub))
    } else {
        HttpResponse::Unauthorized().body("Authorization token missing or invalid")
    }
}

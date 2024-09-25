// src/handlers/mod.rs
pub mod auth;

use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::routes));
}

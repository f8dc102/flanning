// src/handlers/auth/mod.rs
pub mod login;
pub mod logout;
pub mod register;
pub mod unregister;
// pub mod refresh_token;

use actix_web::web;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login)
        .service(register::register)
        .service(logout::logout)
        .service(unregister::unregister);
    // Add refresh_token route if implemented
}

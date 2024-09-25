// src/main.rs
// @TODO: Move Database related code to a separate module.
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use std::io;

// Load Modules
mod handlers;
mod models;

// Define a DbPool type alias
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy().ok();

    // Get DATABASE_URL from environment variables
    let database_url = env::var("DATABASE_URL")?;

    // Create a connection manager
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create a connection pool
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

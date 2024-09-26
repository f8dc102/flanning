// src/main.rs
// Load the modules in the main.rs file
mod handlers;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

// Import the necessary crates
use actix_web::dev::ServiceRequest;
use actix_web::{middleware::Logger, web, App, Error, HttpMessage, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use repositories::database::establish_connection;
use utils::auth::validate_token;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // Validate the token
    match validate_token(credentials.token()) {
        // If the token is valid, attach the claims to the request extensions
        Ok(claims) => {
            // Attach the claims to the request extensions
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        // Return an error if the token is invalid
        Err(_) => {
            let error = actix_web::error::ErrorUnauthorized("Invalid Token");
            Err((error, req))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the environment variables
    dotenvy::dotenv().ok();

    // Initialize the logger
    env_logger::init();

    // Create a connection pool
    let pool = establish_connection();

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(web::scope("/auth").configure(handlers::auth::auth_routes))
            .service(
                web::scope("/user").wrap(HttpAuthentication::bearer(validator)), // Add protected routes here
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

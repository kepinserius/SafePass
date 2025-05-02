use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod controllers;
mod middleware;
mod config;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Get host and port from environment
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_url = format!("{}:{}", host, port);
    
    println!("🚀 Server running at http://{}", server_url);
    
    // Setup governor config for rate limiting
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(20)
        .finish()
        .unwrap();
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(cors)
            // Register routes
            .configure(routes::user_routes::configure)
            .configure(routes::password_routes::configure)
    })
    .bind(server_url)?
    .run()
    .await
} 
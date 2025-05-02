use actix_web::web;

use crate::controllers::user_controller;
use crate::middleware::auth::AuthMiddleware;

// Configure user routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // Base user service
        web::scope("/api/user")
            // Public routes (no auth required)
            .service(
                web::resource("/register")
                    .route(web::post().to(user_controller::register))
            )
            .service(
                web::resource("/login")
                    .route(web::post().to(user_controller::login))
            )
            // Protected routes (auth required)
            .service(
                web::scope("/profile")
                    .wrap(AuthMiddleware)
                    .route("", web::get().to(user_controller::get_profile))
            )
    );
} 
use actix_web::web;

use crate::controllers::password_controller;
use crate::middleware::auth::AuthMiddleware;

// Configure password routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // Base password service (all protected with auth)
        web::scope("/api/passwords")
            .wrap(AuthMiddleware)
            // Get all passwords
            .route("", web::get().to(password_controller::get_all_passwords))
            // Create new password
            .route("", web::post().to(password_controller::create_password))
            // Get specific password
            .route("/{id}", web::get().to(password_controller::get_password))
            // Update password
            .route("/{id}", web::put().to(password_controller::update_password))
            // Delete password
            .route("/{id}", web::delete().to(password_controller::delete_password))
    );
} 
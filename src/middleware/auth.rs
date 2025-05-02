use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;
use uuid::Uuid;

use crate::config::jwt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Get the authorization header
            let auth_header = req
                .headers()
                .get(header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap_or_default())
                .unwrap_or_default();

            // Check if it's a Bearer token
            if !auth_header.starts_with("Bearer ") {
                return Err(ErrorUnauthorized("Invalid authentication token"));
            }

            // Extract the token
            let token = auth_header.trim_start_matches("Bearer ").trim();

            // Validate the token
            match jwt::validate_token(token) {
                Ok(claims) => {
                    // Extract user ID from claims
                    let user_id = match Uuid::parse_str(&claims.user_id) {
                        Ok(id) => id,
                        Err(_) => return Err(ErrorUnauthorized("Invalid user ID in token")),
                    };

                    // Add user_id to request extensions
                    req.extensions_mut().insert(user_id);
                    
                    // Continue with request
                    service.call(req).await
                }
                Err(_) => Err(ErrorUnauthorized("Invalid or expired token")),
            }
        })
    }
} 
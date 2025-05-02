use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

use crate::config::db::DbPool;
use crate::config::jwt;
use crate::models::schema::users;
use crate::models::user::{AuthResponse, LoginUser, RegisterUser, User};

// Register a new user
pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterUser>,
) -> impl Responder {
    // Create new user from registration data
    let new_user = match user_data.to_new_user() {
        Ok(user) => user,
        Err(e) => return HttpResponse::BadRequest().json(e),
    };

    // Insert new user into database
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        // Check if email already exists
        let email_exists = users::table
            .filter(users::email.eq(&user_data.email))
            .select(users::id)
            .first::<Uuid>(&mut conn)
            .optional()?;
            
        if email_exists.is_some() {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                Box::new("Email already registered".to_string()),
            ));
        }
        
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&mut conn)
    })
    .await;

    // Handle insertion result
    match result {
        Ok(db_result) => match db_result {
            Ok(user) => {
                // Generate JWT for the user
                match jwt::generate_token(&user.id.to_string()) {
                    Ok(token) => {
                        let response = AuthResponse {
                            token,
                            user: user.to_response(),
                        };
                        HttpResponse::Created().json(response)
                    }
                    Err(_) => HttpResponse::InternalServerError().json("Token generation failed"),
                }
            },
            Err(e) => {
                if let diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                    HttpResponse::BadRequest().json("Email already registered")
                } else {
                    HttpResponse::InternalServerError().json("Database error")
                }
            },
        },
        Err(e) => {
            // Convert error to a string representation to check contents
            let error_str = format!("{:?}", e);
            if error_str.contains("Email already registered") {
                return HttpResponse::BadRequest().json("Email already registered");
            }
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}

// Login existing user
pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginUser>,
) -> impl Responder {
    let email = login_data.email.clone();
    let password = login_data.password.clone();

    // Find user by email
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn)
            .optional()
    })
    .await;

    // Handle user lookup result
    match result {
        Ok(db_result) => match db_result {
            Ok(user_option) => match user_option {
                Some(user) => {
                    // Verify password
                    if user.verify_password(&password) {
                        // Generate JWT
                        match jwt::generate_token(&user.id.to_string()) {
                            Ok(token) => {
                                let response = AuthResponse {
                                    token,
                                    user: user.to_response(),
                                };
                                HttpResponse::Ok().json(response)
                            }
                            Err(_) => HttpResponse::InternalServerError().json("Token generation failed"),
                        }
                    } else {
                        HttpResponse::Unauthorized().json("Invalid credentials")
                    }
                },
                None => HttpResponse::Unauthorized().json("Invalid credentials"),
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

// Get current user profile
pub async fn get_profile(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    let uuid = user_id.into_inner();

    // Find user by ID
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        users::table
            .find(uuid)
            .first::<User>(&mut conn)
            .optional()
    })
    .await;

    // Handle user lookup result
    match result {
        Ok(db_result) => match db_result {
            Ok(user_option) => match user_option {
                Some(user) => {
                    let user_response = user.to_response();
                    HttpResponse::Ok().json(user_response)
                },
                None => HttpResponse::NotFound().json("User not found"),
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
} 
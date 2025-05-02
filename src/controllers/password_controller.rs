use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::config::db::DbPool;
use crate::models::schema::passwords;
use crate::models::password::{CreatePassword, NewPassword, Password, UpdatePassword};
use crate::utils::encryption;

// Create a new password entry
pub async fn create_password(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    password_data: web::Json<CreatePassword>,
) -> impl Responder {
    // Encrypt the password
    let (encrypted_password, encryption_iv) = match encryption::encrypt_password(&password_data.password) {
        Ok((encrypted, iv)) => (encrypted, iv),
        Err(e) => return HttpResponse::InternalServerError().json(format!("Encryption error: {}", e)),
    };

    let now = Utc::now().naive_utc();
    let user_uuid = user_id.into_inner();
    
    // Create new password entry
    let new_password = NewPassword {
        id: Uuid::new_v4(),
        user_id: user_uuid,
        site_name: password_data.site_name.clone(),
        site_url: password_data.site_url.clone(),
        username: password_data.username.clone(),
        encrypted_password,
        encryption_iv,
        notes: password_data.notes.clone(),
        created_at: now,
        updated_at: now,
    };

    // Insert new password into database
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        diesel::insert_into(passwords::table)
            .values(&new_password)
            .get_result::<Password>(&mut conn)
    })
    .await;

    // Handle insertion result
    match result {
        Ok(db_result) => match db_result {
            Ok(password) => {
                // Decrypt for response
                match encryption::decrypt_password(&password.encrypted_password, &password.encryption_iv) {
                    Ok(decrypted) => {
                        let response = password.to_response(decrypted);
                        HttpResponse::Created().json(response)
                    }
                    Err(e) => HttpResponse::InternalServerError().json(format!("Decryption error: {}", e)),
                }
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to create password entry"),
    }
}

// Get all passwords for current user
pub async fn get_all_passwords(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    let uuid = user_id.into_inner();

    // Fetch all passwords for user
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        passwords::table
            .filter(passwords::user_id.eq(uuid))
            .load::<Password>(&mut conn)
    })
    .await;

    // Handle result
    match result {
        Ok(db_result) => match db_result {
            Ok(passwords) => {
                let mut response_list = Vec::new();
                
                // Decrypt all passwords for response
                for password in passwords {
                    match encryption::decrypt_password(&password.encrypted_password, &password.encryption_iv) {
                        Ok(decrypted) => {
                            response_list.push(password.to_response(decrypted));
                        }
                        Err(_) => {
                            // Skip entries that fail to decrypt
                            continue;
                        }
                    }
                }
                
                HttpResponse::Ok().json(response_list)
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch passwords"),
    }
}

// Get a specific password by ID
pub async fn get_password(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let password_id = path.into_inner();
    let uuid = user_id.into_inner();
    
    // Fetch the specific password
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        passwords::table
            .filter(passwords::id.eq(password_id))
            .filter(passwords::user_id.eq(uuid))
            .first::<Password>(&mut conn)
            .optional()
    })
    .await;

    // Handle result
    match result {
        Ok(db_result) => match db_result {
            Ok(password_option) => match password_option {
                Some(password) => {
                    // Decrypt password for response
                    match encryption::decrypt_password(&password.encrypted_password, &password.encryption_iv) {
                        Ok(decrypted) => {
                            let response = password.to_response(decrypted);
                            HttpResponse::Ok().json(response)
                        }
                        Err(e) => HttpResponse::InternalServerError().json(format!("Decryption error: {}", e)),
                    }
                },
                None => HttpResponse::NotFound().json("Password not found"),
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Database error"),
    }
}

// Update a password
pub async fn update_password(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
    password_data: web::Json<UpdatePassword>,
) -> impl Responder {
    let password_id = path.into_inner();
    let uuid = user_id.into_inner();
    let password_data_inner = password_data.into_inner();
    
    // Update the password
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        // Begin transaction
        conn.transaction(|conn| {
            // Verify password exists
            let existing = passwords::table
                .filter(passwords::id.eq(password_id))
                .filter(passwords::user_id.eq(uuid))
                .first::<Password>(conn)
                .optional()?;
                
            if existing.is_none() {
                return Err(diesel::result::Error::NotFound);
            }
            
            // Prepare update values
            let now = Utc::now().naive_utc();
            
            // Create a new password with updated values
            let mut updated = existing.unwrap();
            updated.updated_at = now;
            
            if let Some(site_name) = password_data_inner.site_name {
                updated.site_name = site_name;
            }
            
            if let Some(site_url) = password_data_inner.site_url {
                updated.site_url = Some(site_url);
            }
            
            if let Some(username) = password_data_inner.username {
                updated.username = username;
            }
            
            if let Some(notes) = password_data_inner.notes {
                updated.notes = Some(notes);
            }
            
            // Update password if provided
            if let Some(new_password) = password_data_inner.password {
                match encryption::encrypt_password(&new_password) {
                    Ok((encrypted, iv)) => {
                        updated.encrypted_password = encrypted;
                        updated.encryption_iv = iv;
                    }
                    Err(_) => return Err(diesel::result::Error::RollbackTransaction),
                }
            }
            
            // Execute update and return updated password
            diesel::update(passwords::table)
                .filter(passwords::id.eq(password_id))
                .filter(passwords::user_id.eq(uuid))
                .set((
                    passwords::site_name.eq(updated.site_name),
                    passwords::site_url.eq(updated.site_url),
                    passwords::username.eq(updated.username),
                    passwords::notes.eq(updated.notes),
                    passwords::encrypted_password.eq(updated.encrypted_password),
                    passwords::encryption_iv.eq(updated.encryption_iv),
                    passwords::updated_at.eq(updated.updated_at)
                ))
                .get_result::<Password>(conn)
        })
    })
    .await;

    // Handle update result
    match result {
        Ok(db_result) => match db_result {
            Ok(updated_password) => {
                // Decrypt for response
                match encryption::decrypt_password(&updated_password.encrypted_password, &updated_password.encryption_iv) {
                    Ok(decrypted) => {
                        let response = updated_password.to_response(decrypted);
                        HttpResponse::Ok().json(response)
                    }
                    Err(e) => HttpResponse::InternalServerError().json(format!("Decryption error: {}", e)),
                }
            },
            Err(e) => {
                if let diesel::result::Error::NotFound = e {
                    HttpResponse::NotFound().json("Password not found")
                } else {
                    HttpResponse::InternalServerError().json("Failed to update password")
                }
            },
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to update password"),
    }
}

// Delete a password
pub async fn delete_password(
    pool: web::Data<DbPool>,
    user_id: web::ReqData<Uuid>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let password_id = path.into_inner();
    let uuid = user_id.into_inner();
    
    // Delete the password
    let result = web::block(move || {
        let conn_result = pool.get();
        if let Err(_) = conn_result {
            return Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new("Database connection error".to_string()),
            ));
        }
        let mut conn = conn_result.unwrap();
        
        diesel::delete(
            passwords::table
                .filter(passwords::id.eq(password_id))
                .filter(passwords::user_id.eq(uuid))
        )
        .execute(&mut conn)
    })
    .await;

    // Handle delete result
    match result {
        Ok(db_result) => match db_result {
            Ok(count) => {
                if count > 0 {
                    HttpResponse::Ok().json("Password deleted successfully")
                } else {
                    HttpResponse::NotFound().json("Password not found")
                }
            },
            Err(_) => HttpResponse::InternalServerError().json("Database error"),
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to delete password"),
    }
} 
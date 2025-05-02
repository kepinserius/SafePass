use crate::models::schema::passwords;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(table_name = passwords)]
#[diesel(belongs_to(crate::models::user::User))]
pub struct Password {
    pub id: Uuid,
    pub user_id: Uuid,
    pub site_name: String,
    pub site_url: Option<String>,
    pub username: String,
    #[serde(skip_serializing)]
    pub encrypted_password: String,
    #[serde(skip_serializing)]
    pub encryption_iv: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = passwords)]
pub struct NewPassword {
    pub id: Uuid,
    pub user_id: Uuid,
    pub site_name: String,
    pub site_url: Option<String>,
    pub username: String,
    pub encrypted_password: String,
    pub encryption_iv: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePassword {
    pub site_name: String,
    pub site_url: Option<String>,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePassword {
    pub site_name: Option<String>,
    pub site_url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PasswordResponse {
    pub id: Uuid,
    pub site_name: String,
    pub site_url: Option<String>,
    pub username: String,
    pub password: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Password {
    pub fn to_response(&self, decrypted_password: String) -> PasswordResponse {
        PasswordResponse {
            id: self.id,
            site_name: self.site_name.clone(),
            site_url: self.site_url.clone(),
            username: self.username.clone(),
            password: decrypted_password,
            notes: self.notes.clone(),
            created_at: self.created_at,
        }
    }
} 
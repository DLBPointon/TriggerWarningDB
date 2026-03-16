use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Users Models
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub password: String, // Remember this is the argon2id hash
    pub admin_access: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

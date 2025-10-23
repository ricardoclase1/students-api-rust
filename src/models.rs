use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct StudentResponse {
    pub id: i64, 
    pub name: String,
    // 🚨 CORRECCIÓN E0277: Cambiamos Option<i32> a Option<i64> para coincidir con SQLite.
    pub age: Option<i64>, 
    pub photo_url: Option<String>,
}

#[derive(Deserialize)] // Solo necesita deserializar desde JSON
pub struct StudentPayload {
    pub name: String,
    pub age: Option<i64>,
    pub photo_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
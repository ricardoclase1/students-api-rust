use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Modelo para la tabla 'users' en DynamoDB
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password_hash: String,
}

// Modelo para la tabla 'students' en DynamoDB
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    #[serde(default = "default_uuid_string")]
    pub id: String,
    pub name: String,
    pub age: i64,
    pub photo_url: Option<String>,
}

// Lo que la API devuelve al listar estudiantes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentResponse {
    pub id: String,
    pub name: String,
    pub age: i64,
    pub photo_url: Option<String>,
}

// Lo que la API recibe para crear/actualizar un estudiante
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentPayload {
    pub name: String,
    pub age: i64,
    pub photo_url: Option<String>,
}

// Petición de login
#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Función auxiliar para generar IDs por defecto
fn default_uuid_string() -> String {
    Uuid::new_v4().to_string()
}
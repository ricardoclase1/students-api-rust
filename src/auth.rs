use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use argon2::{
    Argon2,
    PasswordHasher,
    PasswordVerifier,
    password_hash::{SaltString, PasswordHash, Error as PwHashError}
};
// 🚨 CORRECCIÓN E0432: Importamos OsRng a través de la ruta completa de argon2.
use argon2::password_hash::rand_core::OsRng; 
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// Usamos Result para propagar errores en lugar de `unwrap()`
pub fn create_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims { sub: user_id.to_string(), exp };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn decode_jwt(token: &str, secret: &str) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
}

// Devolvemos un Result para manejar posibles fallos en el hasheo
pub fn hash_password(pw: &str) -> Result<String, PwHashError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(pw.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

/// Verifica una contraseña contra un hash.
/// Devuelve `Ok(true)` si la contraseña es correcta, `Ok(false)` si no lo es,
/// y `Err` si el hash proporcionado es inválido.
pub fn verify_password(hash: &str, pw: &str) -> Result<bool, PwHashError> {
    // `PasswordHash::new` puede fallar si el string del hash no tiene el formato correcto.
    // Propagar este error nos permite detectar problemas de corrupción de datos.
    let parsed_hash = PasswordHash::new(hash)?;

    // `verify_password` devuelve un `Err` si la contraseña no coincide.
    // Lo convertimos a un booleano para una API más clara.
    Ok(Argon2::default().verify_password(pw.as_bytes(), &parsed_hash).is_ok())
}
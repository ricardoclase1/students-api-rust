use actix_web::{dev::Payload, web, Error as ActixError, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use crate::auth::decode_jwt;

// Nuestro "Guard". Si una ruta lo tiene como parámetro,
// el código de `from_request` se ejecutará primero.
pub struct AuthenticationGuard {
    pub _user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Extraemos el secreto JWT del estado de la aplicación, que es más eficiente y seguro.
        let jwt_secret_data = req.app_data::<web::Data<String>>();
        let auth_header = req.headers().get("Authorization");

        // Usamos un `if let` anidado para un código más limpio y seguro
        if let (Some(secret), Some(auth_header)) = (jwt_secret_data, auth_header) {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str["Bearer ".len()..];

                    if let Ok(decoded) = decode_jwt(token, secret.get_ref()) {
                        return ok(AuthenticationGuard {
                        _user_id: decoded.claims.sub,
                        });
                    }
                }
            }
        }

        // Si algo falla, devolvemos un error de "No Autorizado".
        err(actix_web::error::ErrorUnauthorized("Invalid token"))
    }
}
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::{LoginRequest, StudentPayload, StudentResponse};
use crate::middleware::AuthenticationGuard;
use crate::auth::{create_jwt, verify_password, hash_password};
use serde_json::json;

// Sugerencia: Crear un tipo de error personalizado para la API mejora el manejo de errores.
// (Esto requeriría un nuevo módulo de error, pero por ahora usaremos un mapeo simple).

#[post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<SqlitePool>,
    jwt_secret: web::Data<String>,
) -> impl Responder {
    // Usamos `fetch_optional` que devuelve un `Option` y es más seguro si el usuario no existe.
    let user_result = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = ?",
        req.username
    )
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user_result {
        Ok(Some(user)) => user,
        // Si el usuario no existe o hay un error de BD, devolvemos Unauthorized.
        _ => return HttpResponse::Unauthorized().finish(),
    };

    // Mejora: Validar que el ID del usuario no sea nulo. Un usuario siempre debe tener ID.
    // Si `id` es `None`, algo está mal en la base de datos o en la lógica.
    let user_id = match user.id {
        Some(id) => id,
        None => return HttpResponse::InternalServerError().json("Error: User ID is missing"),
    };

    // Usamos la nueva función verify_password que devuelve un Result
    match verify_password(&user.password_hash, &req.password) {
        Ok(true) => {
            // El secreto ahora viene del estado de la aplicación
            match create_jwt(&user_id.to_string(), jwt_secret.get_ref()) {
                Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
                Err(_) => HttpResponse::InternalServerError().json("Error al crear el token"),
            }
        }
        // Contraseña incorrecta o hash inválido
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[post("/register")]
pub async fn register(
    req: web::Json<LoginRequest>,
    pool: web::Data<SqlitePool>,
) -> impl Responder {
    // Hasheamos la contraseña antes de guardarla
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Error hashing password"}));
        }
    };

    // Insertamos el nuevo usuario en la base de datos
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        req.username,
        password_hash
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "User created"})),
        Err(_) => HttpResponse::Conflict().json(json!({"status": "error", "message": "Username already exists"})),
    }
}

#[get("/students")]
pub async fn list_students(
    pool: web::Data<SqlitePool>,
    _guard: AuthenticationGuard, // <-- ¡LA MAGIA ESTÁ AQUÍ!
) -> impl Responder {
    // Mejora: Manejar el error de la base de datos en lugar de usar `.unwrap()`.
    match sqlx::query_as!(StudentResponse, "SELECT id, name, age, photo_url FROM students")
        .fetch_all(pool.get_ref()).await
    {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(_) => HttpResponse::InternalServerError().json("Error fetching students"),
    }
}

#[post("/students")]
pub async fn create_student(
    payload: web::Json<StudentPayload>,
    pool: web::Data<SqlitePool>,
    _guard: AuthenticationGuard, // Protegemos la ruta
) -> impl Responder {
    // La macro query! verificará la consulta en tiempo de compilación
    let result = sqlx::query!(
        "INSERT INTO students (name, age, photo_url) VALUES (?, ?, ?)",
        payload.name,
        payload.age,
        payload.photo_url
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "Student created"})),
        Err(_) => HttpResponse::InternalServerError().json("Error creating student"),
    }
}

#[put("/students/{id}")]
pub async fn update_student(
    path: web::Path<i64>,
    payload: web::Json<StudentPayload>,
    pool: web::Data<SqlitePool>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let student_id = path.into_inner();
    let result = sqlx::query!(
        "UPDATE students SET name = ?, age = ?, photo_url = ? WHERE id = ?",
        payload.name,
        payload.age,
        payload.photo_url,
        student_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        // Si se afectó al menos una fila, fue un éxito.
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().json(json!({"status": "success"})),
        // Si no se afectó ninguna fila, el estudiante no existía.
        Ok(_) => HttpResponse::NotFound().json(json!({"error": "Student not found"})),
        Err(_) => HttpResponse::InternalServerError().json("Error updating student"),
    }
}

#[delete("/students/{id}")]
pub async fn delete_student(
    path: web::Path<i64>,
    pool: web::Data<SqlitePool>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let student_id = path.into_inner();
    let result = sqlx::query!("DELETE FROM students WHERE id = ?", student_id)
        .execute(pool.get_ref())
        .await;

    match result {
        // Si se afectó al menos una fila, fue un éxito.
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({ "status": "success", "message": "Student deleted" }))
        }
        // Si no se afectó ninguna fila, el estudiante no existía.
        Ok(_) => HttpResponse::NotFound().json(json!({ "error": "Student not found" })),
        Err(_) => HttpResponse::InternalServerError().json("Error deleting student"),
    }
}
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use aws_sdk_dynamodb::Client as DynamoDbClient;
use aws_sdk_dynamodb::types::AttributeValue;
use serde_json::json;
use uuid::Uuid;
use crate::models::{LoginRequest, StudentPayload, Student, StudentResponse, User};
use crate::middleware::AuthenticationGuard;
use crate::auth::{create_jwt, verify_password, hash_password};

#[post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    client: web::Data<DynamoDbClient>,
    jwt_secret: web::Data<String>,
) -> impl Responder {
    let result = match client
        .get_item()
        .table_name("users")
        .key("username", AttributeValue::S(req.username.clone()))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            log::error!("Error en DynamoDB al obtener usuario: {}", e);
            return HttpResponse::InternalServerError().json("Error de base de datos");
        }
    };
    
    let user_item = match result.item {
        Some(item) => item,
        None => return HttpResponse::Unauthorized().json("Usuario no encontrado"),
    };

    let user: User = match serde_dynamo::from_item(user_item) {
        Ok(u) => u,
        Err(e) => {
            log::error!("Error al deserializar usuario desde DynamoDB: {}", e);
            return HttpResponse::InternalServerError().json("Error procesando datos de usuario");
        }
    };

    match verify_password(&user.password_hash, &req.password) {
        Ok(true) => {
            match create_jwt(&user.username, jwt_secret.get_ref()) {
                Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
                Err(_) => HttpResponse::InternalServerError().json("Error al crear el token"),
            }
        }
        _ => HttpResponse::Unauthorized().json("Credenciales inválidas"),
    }
}

#[post("/register")]
pub async fn register(
    req: web::Json<LoginRequest>,
    client: web::Data<DynamoDbClient>,
) -> impl Responder {
    let password_hash = match hash_password(&req.password) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json("Error al hashear la contraseña"),
    };

    let new_user = User {
        username: req.username.clone(),
        password_hash,
    };

    let user_item = match serde_dynamo::to_item(new_user) {
        Ok(item) => item,
        Err(_) => return HttpResponse::InternalServerError().json("Error al preparar datos de usuario"),
    };

    let result = client
        .put_item()
        .table_name("users")
        .set_item(Some(user_item))
        .condition_expression("attribute_not_exists(username)")
        .send()
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "User created"})),
        Err(_) => HttpResponse::Conflict().json(json!({"status": "error", "message": "Username already exists"})),
    }
}

#[get("/students")]
pub async fn list_students(
    client: web::Data<DynamoDbClient>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let result = match client.scan().table_name("students").send().await {
        Ok(res) => res,
        Err(_) => return HttpResponse::InternalServerError().json("Error al obtener estudiantes"),
    };
    
    let students: Vec<StudentResponse> = result
        .items
        .unwrap_or_default()
        .into_iter()
        .filter_map(|item| serde_dynamo::from_item(item).ok())
        .collect();
        
    HttpResponse::Ok().json(students)
}

#[post("/students")]
pub async fn create_student(
    payload: web::Json<StudentPayload>,
    client: web::Data<DynamoDbClient>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let new_student = Student {
        id: Uuid::new_v4().to_string(), // DynamoDB no genera IDs, los creamos nosotros
        name: payload.name.clone(),
        age: payload.age,
        photo_url: payload.photo_url.clone(),
    };

    let student_item = match serde_dynamo::to_item(new_student) {
        Ok(item) => item,
        Err(_) => return HttpResponse::InternalServerError().json("Error al preparar datos"),
    };

    let result = client
        .put_item()
        .table_name("students")
        .set_item(Some(student_item))
        .send()
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({"status": "success", "message": "Student created"})),
        Err(_) => HttpResponse::InternalServerError().json("Error al crear el estudiante"),
    }
}

#[put("/students/{id}")]
pub async fn update_student(
    path: web::Path<String>,
    payload: web::Json<StudentPayload>,
    client: web::Data<DynamoDbClient>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let student_id = path.into_inner();
    
    let result = client
        .update_item()
        .table_name("students")
        .key("id", AttributeValue::S(student_id))
        .update_expression("SET #n = :name, #a = :age, #p = :photo_url")
        .expression_attribute_names("#n", "name")
        .expression_attribute_names("#a", "age")
        .expression_attribute_names("#p", "photo_url")
        .expression_attribute_values(":name", AttributeValue::S(payload.name.clone()))
        .expression_attribute_values(":age", AttributeValue::N(payload.age.to_string()))
        .expression_attribute_values(":photo_url", AttributeValue::S(payload.photo_url.clone().unwrap_or_default()))
        .send()
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(_) => HttpResponse::InternalServerError().json("Error al actualizar el estudiante"),
    }
}

#[delete("/students/{id}")]
pub async fn delete_student(
    path: web::Path<String>,
    client: web::Data<DynamoDbClient>,
    _guard: AuthenticationGuard,
) -> impl Responder {
    let student_id = path.into_inner();
    
    let result = client
        .delete_item()
        .table_name("students")
        .key("id", AttributeValue::S(student_id))
        .send()
        .await;
    
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success", "message": "Student deleted" })),
        Err(_) => HttpResponse::InternalServerError().json("Error al eliminar el estudiante"),
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login)
            .service(register)
            .service(list_students)
            .service(create_student)
            .service(update_student)
            .service(delete_student),
    );
}
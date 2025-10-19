pub mod db;
pub mod auth;
mod handlers;
mod middleware;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use db::init_db;


pub async fn run() -> std::io::Result<()> {
    // Inicializa el logger. Esto nos permitirá ver las peticiones entrantes.
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();

    // Cargamos las variables de entorno de forma segura
    let db_url = std::env::var("DATABASE_URL")
        .expect("❌ Error: La variable DATABASE_URL no está definida en el archivo .env.");
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("❌ Error: La variable JWT_SECRET no está definida en el archivo .env.");

    let pool = init_db(&db_url).await;

    println!("🚀 Servidor Actix corriendo en http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // <-- Añadimos el middleware de logging
            .app_data(web::Data::new(jwt_secret.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::login)
            .service(handlers::list_students)
            .service(handlers::create_student)
            .service(handlers::update_student)
            .service(handlers::delete_student)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

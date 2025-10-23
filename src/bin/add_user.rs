use students_api::auth::hash_password;
use students_api::db::init_db;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // Cargamos las variables de entorno del archivo .env
    if dotenv::dotenv().is_err() {
        eprintln!("❌ Error: No se pudo encontrar el archivo .env. Asegúrate de que exista en la raíz del proyecto.");
        return;
    }

    let db_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("❌ Error: La variable DATABASE_URL no está definida en el archivo .env.");
            return;
        }
    };

    // Nos conectamos a la base de datos
    let pool = init_db(&db_url).await;

    println!("--- Creando un nuevo usuario ---");

    print!("Introduce el nombre de usuario: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Introduce la contraseña: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    let password_hash = match hash_password(password) {
        Ok(hash) => hash,
        Err(_) => {
            eprintln!("\n❌ Error: No se pudo generar el hash de la contraseña.");
            return;
        }
    };

    match sqlx::query!("INSERT INTO users (username, password_hash) VALUES (?, ?)", username, password_hash)
        .execute(&pool)
        .await {
        Ok(_) => println!("\n✅ ¡Usuario '{}' creado con éxito!", username),
        Err(e) => eprintln!("\n❌ Error al crear el usuario: {}", e),
    }
}
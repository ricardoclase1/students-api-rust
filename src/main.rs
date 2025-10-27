    use lambda_http::{run, service_fn, Body, Error, Request, Response, IntoResponse};
    use std::sync::Arc;
    use aws_sdk_dynamodb::Client as DynamoDbClient;

    // Importa los módulos directamente aquí
    pub mod auth;
    pub mod handlers;
    pub mod middleware;
    pub mod models;
    pub mod routes;
    pub mod utils;

    use utils::ddb::DDBRepository;

    #[tokio::main]
    async fn main() -> Result<(), Error> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .without_time()
            .init();

        // --- CONFIGURACIÓN DE AWS A PRUEBA DE FALLOS ---
        // En lugar de adivinar, forzamos la región directamente.
        let region = aws_sdk_dynamodb::config::Region::new("us-east-1");
        let config = aws_config::from_env().region(region).load().await;
        // --------------------------------------------------------------------

        let dynamodb_client = DynamoDbClient::new(&config);
        let ddb_repo = DDBRepository::new(dynamodb_client.clone(), "users".to_string(), "students".to_string());
        let ddb_repo_arc = Arc::new(ddb_repo);

        run(service_fn(move |event: Request| {
            let ddb_repo_clone = ddb_repo_arc.clone();
            async move {
                let path = event.uri().path();
                match path {
                    "/v1/register" => handlers::auth_handler::register(ddb_repo_clone.user_repo(), event).await,
                    "/v1/login" => handlers::auth_handler::login(ddb_repo_clone.user_repo(), event).await,
                    // Aquí puedes añadir el resto de tus rutas
                    _ => Ok(Response::builder().status(404).body("Not Found".into()).unwrap())
                }
            }
        })).await
    }
    
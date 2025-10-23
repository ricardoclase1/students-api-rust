use students_api::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Llama a la función principal desde nuestra librería
    run().await
}
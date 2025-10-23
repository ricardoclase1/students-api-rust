# --- Etapa 1: Builder ---
# Usamos una imagen oficial de Rust para compilar el proyecto.
# Esto mantiene la imagen final pequeña, ya que las herramientas de compilación no estarán en ella.
FROM rust:1.79-slim-bookworm AS builder

# Instalamos las dependencias necesarias para compilar `sqlx` con SQLite.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiamos solo los archivos de dependencias para aprovechar el caché de Docker.
# Si no cambian, Docker no volverá a descargar las dependencias.
# --- SOLUCIÓN DEFINITIVA ---
# Ignoramos el Cargo.lock local y forzamos a Cargo a generar uno nuevo desde cero.
COPY Cargo.toml .

# Creamos un proyecto falso para que `cargo` pueda resolver y compilar las dependencias.
RUN mkdir src && echo "pub fn run() -> std::io::Result<()> { Ok(()) }" > src/lib.rs
RUN cargo build --release

# Ahora que las dependencias están cacheadas, borramos el `src` falso.
RUN rm -rf src

# Copiamos el código fuente real y las migraciones.
COPY src ./src
COPY migrations ./migrations

# Compilamos la aplicación final en modo 'release' para optimizarla.
RUN cargo build --release --bin students_api

# --- Etapa 2: Runtime ---
# Usamos la imagen base oficial de AWS Lambda para "provided runtimes".
# Es una imagen muy ligera diseñada para ejecutar binarios personalizados.
FROM public.ecr.aws/lambda/provided:al2023

# Copiamos el binario compilado desde la etapa 'builder'.
# AWS Lambda espera que el ejecutable se llame 'bootstrap' y esté en /var/task/.
COPY --from=builder /app/target/release/students_api /var/task/bootstrap

# Copiamos las migraciones para que la Lambda pueda ejecutarlas si es necesario.
COPY --from=builder /app/migrations /var/task/migrations

# El comando que Lambda ejecutará. En este caso, nuestro binario.
CMD ["bootstrap"]
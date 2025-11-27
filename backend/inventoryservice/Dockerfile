# Etapa 1: Chef - Preparación para cachear dependencias
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

# Etapa 2: Planner - Crear el plan de dependencias
FROM chef AS planner
COPY . .
# Generar el plan de dependencias para cachear
RUN cargo chef prepare --recipe-path recipe.json

# Etapa 3: Builder - Compilar dependencias y el proyecto
FROM chef AS builder
# Copiar el plan de dependencias
COPY --from=planner /app/recipe.json recipe.json
# Compilar las dependencias (capa de cache)
RUN cargo chef cook --release --recipe-path recipe.json

# Copiar el resto del código fuente
COPY . .
# Copiar datos de sqlx para compilación offline
COPY .sqlx .sqlx
# Habilitar modo offline de sqlx
ENV SQLX_OFFLINE=true
# Compilar el proyecto
RUN cargo build --release

# Etapa 4: Runtime - Imagen final de ejecución
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Instalar dependencias mínimas
RUN apt-get update && apt-get install -y --no-install-recommends tini && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado desde la etapa de builder
COPY --from=builder /app/target/release/inventory_service .



# Exponer el puerto que usa la aplicación
EXPOSE 3000

# Usar tini para gestionar el proceso de la aplicación
ENTRYPOINT ["/usr/bin/tini", "--"]

# Comando para ejecutar la aplicación
CMD ["./inventory_service"]
# ---- Builder Stage ----
# Use the official Rust image as a build environment
FROM rust:1.78 as builder

# Set working directory for the builder
WORKDIR /usr/src/app

# Copy Cargo.toml to cache dependencies. Cargo.lock should also be copied if present.
# We create a dummy src/main.rs first to allow 'cargo build' to cache dependencies.
COPY productservice/Cargo.toml ./productservice/Cargo.toml
# If you have a Cargo.lock, uncomment the next line:
# COPY productservice/Cargo.lock ./productservice/Cargo.lock

WORKDIR /usr/src/app/productservice
# Create a dummy src/main.rs to allow `cargo build` to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build only the dependencies. This command may fail if there are no dependencies
# or if the dummy main.rs is not sufficient. In a real project, consider
# `cargo check --release` or `cargo build --release --no-run` to pre-cache.
RUN cargo build --release || true 

# Clean up the dummy main.rs and any resulting binaries
RUN rm src/main.rs && rm -rf target/release/productservice

# Copy the actual source code
COPY productservice/src ./src

# Build the application
RUN cargo build --release


# ---- Final Stage ----
# Use a minimal image for the final container
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS (Rust's reqwest, etc., need this)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/productservice/target/release/productservice /usr/local/bin/productservice

# The application expects db/temp.json. This assumes that `db/temp.json` is at the root
# of the docker build context, and the productservice binary runs from `/app`.
# The path `../db/temp.json` used in Rust code resolves to `/db/temp.json` in the container
# if the binary is run from `/app`.
WORKDIR /app
COPY db/temp.json /db/temp.json

# Expose the port the app runs on
EXPOSE 3002

# Set the entrypoint
CMD ["/usr/local/bin/productservice"]
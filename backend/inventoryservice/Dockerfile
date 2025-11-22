# ---- Builder Stage ----
# Use the official Rust image as a build environment
FROM rust:1.78 as builder

# Set the working directory
WORKDIR /app

# Copy the dependency manifest files
COPY Cargo.toml Cargo.lock ./

# Build dependencies, this will cache them
# Create a dummy main.rs to build only dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY src ./src

# Build the application
# Remove the dummy file and rebuild
RUN rm -f src/main.rs
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal image for the final container
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS (needed by reqwest)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/inventoryservice /usr/local/bin/inventoryservice

# Set the working directory
WORKDIR /app

# Expose the port the app runs on, matching docker-compose.yml
EXPOSE 3001

# Set the entrypoint
CMD ["/usr/local/bin/inventoryservice"]
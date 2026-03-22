# Stage 1: Build the Rust binary
# Use latest stable rust to support Edition 2024 dependencies
FROM rust:1.85-slim AS builder

WORKDIR /usr/src/where-in-pi
COPY . .

# Build for release to ensure maximum search speed
RUN cargo build --release

# Stage 2: Final lightweight image
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary SSL certs
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/where-in-pi/target/release/where-in-pi /usr/local/bin/where-in-pi

# Copy pre-generated data and UI assets (using exact UI case)
COPY data/ ./data/
COPY UI/ ./static/

# Render default port
EXPOSE 10000

CMD ["where-in-pi"]

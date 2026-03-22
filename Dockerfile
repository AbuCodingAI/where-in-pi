# Stage 1: Build the Rust binary
# Using 1.94 to fully support Edition 2024 and modern dependencies
FROM rust:1.94-slim AS builder

WORKDIR /usr/src/where-in-pi
COPY . .

# Build for release
RUN cargo build --release

# Stage 2: Final lightweight image
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary SSL certs
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/where-in-pi/target/release/where-in-pi /usr/local/bin/where-in-pi

# Copy pre-generated data and UI assets (using UI caps to match your repo)
COPY data/ ./data/
COPY UI/ ./static/

# Render default port
EXPOSE 10000

CMD ["where-in-pi"]

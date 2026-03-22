# Stage 1: Build the Rust binary
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/where-in-pi
COPY . .

# Build for release to ensure maximum search speed
RUN cargo build --release

# Stage 2: Final lightweight image
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary SSL certs for any external API calls
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/where-in-pi/target/release/where-in-pi /usr/local/bin/where-in-pi

# Copy your pre-generated Pi data and static web files
# NOTE: Source is 'ui/' to match your project structure, mapped to '/app/static' in container
COPY data/ ./data/
COPY UI/ ./static/

# Render uses port 10000 by default
EXPOSE 10000

# Start the high-performance backend
CMD ["where-in-pi"]


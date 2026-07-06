# syntax=docker/dockerfile:1
# Rhyph — self-hosted federated ticketing platform

FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml crates/core/Cargo.toml
COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/federation/Cargo.toml crates/federation/Cargo.toml
COPY crates/server/Cargo.toml crates/server/Cargo.toml
COPY crates/payments/Cargo.toml crates/payments/Cargo.toml

# Create dummy source files for layer caching
RUN mkdir -p crates/core/src crates/api/src crates/federation/src crates/server/src crates/payments/src && \
    echo "fn main() {}" > crates/server/src/main.rs && \
    echo "" > crates/core/src/lib.rs && \
    echo "" > crates/api/src/lib.rs && \
    echo "" > crates/federation/src/lib.rs && \
    echo "" > crates/payments/src/lib.rs

# Fetch deps (cached layer)
RUN cargo build --release && rm -rf target/release/deps/rhyph*

# Copy real source
COPY crates/ crates/

# Build the actual binary
RUN cargo build --release -p rhyph-server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates postgresql-client && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rhyph-server /usr/local/bin/rhyph-server
COPY crates/core/src/db/migrations /migrations
COPY scripts/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

RUN chmod +x /usr/local/bin/docker-entrypoint.sh

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

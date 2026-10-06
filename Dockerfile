# Stage 1: Build the Rust ERP binary
FROM rust:1.82-slim-bookworm AS builder

WORKDIR /usr/src/erp

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests and source code
COPY Cargo.toml ./
COPY crates ./crates

# Build optimized release binary
RUN cargo build --release --bin erp-api

# Stage 2: Minimal distroless runtime image (<25MB, non-root user)
FROM gcr.io/distroless/cc-debian12:nonroot

WORKDIR /app

COPY --from=builder /usr/src/erp/target/release/erp-api /app/erp-api

USER nonroot:nonroot
EXPOSE 8080

ENTRYPOINT ["/app/erp-api"]

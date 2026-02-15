# syntax=docker/dockerfile:1.6

ARG RUST_VERSION=1.76
FROM rust:${RUST_VERSION}-slim AS base
WORKDIR /app
ENV SQLX_OFFLINE=true

FROM base AS chef
RUN cargo install cargo-chef
COPY backend/ /app/backend/
WORKDIR /app/backend
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev clang && rm -rf /var/lib/apt/lists/*
COPY --from=chef /app/backend/recipe.json /app/backend/recipe.json
WORKDIR /app/backend
RUN cargo chef cook --release --recipe-path recipe.json
COPY backend/ /app/backend/
RUN cargo build --release --bin devflow-backend

FROM gcr.io/distroless/cc-debian12:latest AS runtime
WORKDIR /app
ENV APP_ENV=production \
    RUST_LOG=info,devflow_backend=info \
    SQLX_OFFLINE=true
USER 1000
COPY --from=builder /app/backend/target/release/devflow-backend /usr/local/bin/devflow-backend
EXPOSE 8000
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s \
  CMD [ "wget", "-qO-", "http://127.0.0.1:8000/health" ]
CMD ["/usr/local/bin/devflow-backend"]

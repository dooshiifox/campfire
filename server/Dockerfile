# https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/
FROM rust:alpine AS builder
WORKDIR /app
RUN apk update && apk upgrade
RUN apk add --no-cache mold gcc lld musl-dev
RUN rustup toolchain install nightly
RUN rustup default nightly
ENV SQLX_OFFLINE true
COPY server/Cargo.toml server/Cargo.toml
COPY database/Cargo.toml database/Cargo.toml
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .cargo .cargo
COPY migrations migrations
# Create dummy src files to build dependencies
RUN mkdir -p server/src && echo "fn main() {}" > server/src/main.rs
RUN mkdir -p database/src && echo "fn dummy() {}" > database/src/lib.rs
# Run initial build for dependencies
RUN cargo check
# Copy source code
COPY server server
COPY database database
RUN cargo build --release --bin server

FROM alpine AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/server server
ENTRYPOINT [ "./server" ]

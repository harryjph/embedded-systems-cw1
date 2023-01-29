FROM rust:alpine AS build
# musl-dev is required in order to dynamically link proc_macro crates
# openssl-dev is required for SeaORM
RUN apk add --no-cache musl-dev protoc openssl-dev

WORKDIR /build
# Download and compile dependencies first so that they get cached
COPY Cargo.* ./
RUN mkdir src \
    && echo "fn main() {}" > build.rs \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -r Cargo.* src/ build.rs
# Build the actual project
COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin
COPY --from=build /build/target/release/server .
ENV CONFIG_PATH=/etc/server/config.toml
ENV DATABASE_PATH=/etc/server/database.db
VOLUME /etc/server/

ENTRYPOINT ["server"]

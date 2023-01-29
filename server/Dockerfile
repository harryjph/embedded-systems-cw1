FROM rust:alpine AS build
# musl-dev is required in order to dynamically link proc_macro crates
# openssl-dev is required for SeaORM
RUN apk add --no-cache musl-dev protoc openssl-dev
WORKDIR /build
# Build the project
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release

FROM alpine:latest
WORKDIR /usr/local/bin
COPY --from=build /build/target/release/server .
ENV CONFIG_PATH=/etc/server/config.toml
ENV DATABASE_PATH=/etc/server/database.db
VOLUME /etc/server/
ENTRYPOINT ["server"]

FROM rust:alpine AS build
# musl-dev is required in order to dynamically link proc_macro crates
RUN apk add --no-cache musl-dev protoc openssl

WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin
COPY --from=build /build/target/release/server .
EXPOSE 80
ENV CONFIG_PATH=/etc/server/config.toml
ENV DATABASE_PATH=/etc/server/database.db
VOLUME /etc/server/config.toml
VOLUME /etc/server/database.db

ENTRYPOINT ["server"]

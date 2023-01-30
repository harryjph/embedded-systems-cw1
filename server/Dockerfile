FROM rust:alpine AS build
# musl-dev is required in order to dynamically link proc_macro crates
# openssl-dev is required for SeaORM
RUN apk add --no-cache musl-dev protoc
WORKDIR /build
# Build the project
COPY . .
ENV CARGO_TERM_COLOR=always
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && cp target/release/server .

FROM alpine:latest
WORKDIR /usr/local/bin
COPY --from=build /build/server .
ENV CONFIG_PATH=/etc/server/config.toml
ENV DATABASE_PATH=/etc/server/database.db
VOLUME /etc/server/
ENTRYPOINT ["server"]

FROM rust:alpine AS build
# musl-dev is required in order to dynamically link proc_macro crates
RUN apk add --no-cache musl-dev protoc

WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin
COPY --from=build /build/target/release/server .
EXPOSE 80

ENTRYPOINT ["server"]

FROM rust:alpine AS build

WORKDIR /build
COPY . .
RUN cargo build --release

FROM alpine:latest

WORKDIR /usr/local/bin
COPY --from=build /build/target/release/server .
EXPOSE 80

ENTRYPOINT ["server"]

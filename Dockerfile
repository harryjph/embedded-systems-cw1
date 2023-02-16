FROM node:alpine AS frontend-build
WORKDIR /app
COPY frontend .
RUN npm install && npm run build

FROM rust:alpine AS server-build
# musl-dev is required in order to dynamically link proc_macro crates
RUN apk add --no-cache musl-dev protoc
WORKDIR /build
# Build the project
COPY server .
ENV CARGO_TERM_COLOR=always
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && cp target/release/server .

FROM node:alpine AS marketing-build
WORKDIR /app
COPY marketing .
RUN npm install && npm run build

FROM alpine:latest
WORKDIR /usr/local/bin
COPY --from=frontend-build /app/build /etc/frontend
COPY --from=marketing-build /app/dist /etc/marketing
COPY --from=server-build /build/server .
ENV CONFIG_PATH=/etc/server/config.toml
ENV DATABASE_PATH=/etc/server/database.db
ENV FRONTEND_PATH=/etc/frontend
ENV MARKETING_PATH=/etc/marketing
VOLUME /etc/server/
ENTRYPOINT ["server"]

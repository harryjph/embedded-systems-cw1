# Server

This directory contains the application that runs on remote server.

# Build Requirements

## Running Locally

* Rust Toolchain installed (`rustup`, `cargo`, `rustc` commands available)
* Protocol Buffers Compiler (`protoc`) installed

## Running in Docker

* Docker installed (`docker` command available)

# Building

* Build local executable: `cargo build` or `cargo build --release`
* Build Docker image: `docker build .`

# Testing

`cargo test`

# Server

This directory contains the application that runs on remote server.

# Build Requirements

## Running Locally

* Rust Toolchain installed (`rustup`, `cargo`, `rustc` commands available)
* Protocol Buffers Compiler (`protoc`) installed

# Running Diesel for orm
* Sqlite3 (`sqlite3-dev`) installed: `sudo apt install libsqlite3-dev`
* (`libpq-dev`) installed: `sudo apt install libpq-dev`
* libmysqlclient-dev installed: `sudo apt install libmysqlclient-dev`
* install diesel with: `cargo install diesel_cli`

* Run `diesel migration redo` to instantiate the database

## Running in Docker

* Docker installed (`docker` command available)

# Building

* Build local executable: `cargo build` or `cargo build --release`
* Build Docker image: `docker build .`

# Testing

`cargo test`

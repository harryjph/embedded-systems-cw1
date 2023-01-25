# Raspberry Pi App

This directory contains the application that runs on the RPi.

# Build Requirements

* Rust Toolchain installed (`rustup`, `cargo`, `rustc` commands available)
* Appropriate Rust Target installed (`rustup target list --installed` contains `arm-unknown-linux-musleabihf`)
  * `rustup target add arm-unknown-linux-musleabihf`
* ARMv6 Hard Float GCC installed (`arm-none-linux-gnueabihf-gcc` command available)
  * On Debian-based Linux: `apt install gcc-arm-linux-gnueabihf`
  * On Windows: [Download from ARM (arm-none-linux-gnueabihf)](https://developer.arm.com/downloads/-/gnu-a), add `bin` directory in extracted download to `%PATH%`

# Building

`cargo build` or `cargo build --release`

# Raspberry Pi App

This directory contains the application that runs on the RPi.

# Build Requirements

* Rust Toolchain installed (`rustup`, `cargo`, `rustc` commands available)
* Appropriate Rust Target installed (`rustup target list --installed` contains `arm-unknown-linux-musleabihf`)
  * `rustup target add arm-unknown-linux-musleabihf`
* ARMv6 Hard Float C compiler installed and available to Rust
  * On Raspberry Pi: If you have `cc` available, do nothing.
  * On Debian-based Linux (Cross Compiling):
    * `apt install gcc-arm-linux-gnueabihf`
    * Add the following to `~/.cargo/config.toml`:
      ```
      [target.arm-unknown-linux-musleabihf]
      linker = "arm-linux-gnueabihf-gcc"
      ```
  * On Windows (Cross Compiling):
    * [Download from ARM (arm-none-linux-gnueabihf)](https://developer.arm.com/downloads/-/gnu-a), extract somewhere, add top level `bin` directory to `%PATH%`
    * Add the following to `~/.cargo/config.toml`:
      ```
      [target.arm-unknown-linux-musleabihf]
      linker = "arm-none-linux-gnueabihf-gcc"
      ```

# Building

`cargo build` or `cargo build --release`

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
* Protocol Buffers Compiler (`protoc`) installed
  * On Debian-based Linux: `apt install protobuf-compiler`
* (For distributions) `dpkg-deb` installed

# Building

* Debug binary: `cargo build`
* Release binary: `cargo build --release`
* Debian package: `./package.sh`. This can be installed on the Raspberry Pi using: `dpkg -i rpi-app.deb` and uninstalled using `dpkg --purge rpi-app` 

# Testing

`cargo test`

## Testing Requirements

Must be tested under Linux. Requires `qemu-arm-static` to be installed.
* On Debian-based Linux: `apt install qemu-user-static`
* On WSL2: WSL2 Packages do not automatically configure binfmt. Add the runner manually to cargo's config:
  * `apt install qemu-user-static`
  * Modify `~/.cargo/config.toml`:
    ```
    [target.arm-unknown-linux-musleabihf]
    linker = "arm-linux-gnueabihf-gcc"
    runner = "qemu-arm-static"    
    ```


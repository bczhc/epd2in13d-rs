epd2in13d-rs
====

Control Waveshare EPD2in13D E-Ink screen on Raspberry Pi, with Rust!!

## Build

First go to project [epd2in13d](https://github.com/bczhc/epd2in13d) and build it. After the build,
you will get a shared library located at `build/libepd2in13d.so`.

Copy the shared library to `libs` (create the folder if it doesn't exist) under the project root.

Now do the cross-compilation:
```bash
cargo build --target aarch64-unknown-linux-gnu
```

Don't forget to install Rust target `aarch64-unknown-linux-gnu` before it. Also, cross-compilation
toolchains are required. E.g. on Arch Linux, they're from the package `aarch64-linux-gnu-gcc`.

Cross-compilation toolchains are specified in `.cargo/config`, you may modify them as needed.

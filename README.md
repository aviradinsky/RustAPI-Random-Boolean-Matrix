# A simple RestAPI implemented in Rust

Nothing major, just a small project to learn the basics of web development and [Rust](https://www.rust-lang.org/).

## What this is

This RestAPI provides a 2D array of random booleans (represented as one's and zeros) based on the ***columns*** and ***rows*** specified in the URL.

## Build Instructions
Requirements: [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
To run the release optimized version (longer compile times):
```bash
# Clone this repo
cd RustAPI-Random-Boolean-Matrix
cargo build --release
./target/release/rock
```
To run with a quicker compile time:
```bash
# Clone this repo
cd RustAPI-Random-Boolean-Matrix
cargo run
```
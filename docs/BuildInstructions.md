# Building rc from Source

Below are instructions on how to build Deno from source. We welcome and appreciate all contributions to RC.
This page serves as a helper to get you started on contributing.

## Cloning the Repository

```sh
git clone https://github.com/castrogarciajs/rc.git
```

## Prerequisites

### Rust

[Update or Install Rust](https://www.rust-lang.org/tools/install). Check that Rust installed/updated correctly:

```sh
rustc -V
cargo -V
```

## Building

Build with Cargo:

````sh
cargo build

# Build errors?  Ensure you have latest main and try building again, or if that doesn't work try:
cargo clean && cargo build

# Run:
./target/debug/rc --get https://jsonplaceholder.typicode.com/posts/1
```

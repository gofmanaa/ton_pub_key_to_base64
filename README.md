# Rust Base64 Public Key Encoder

This is a Rust application that reads a public key file, trims the first 4 bytes, and outputs the Base64-encoded public key. It also includes an optional `--verbose` flag to provide additional output.

## Features

- Reads a public key file in binary format.
- Removes the first 4 bytes from the public key file.
- Outputs the Base64-encoded public key.
- Supports a `--verbose` flag for additional output.

## Dependencies

- [Clap](https://crates.io/crates/clap): For parsing command-line arguments.
- [Base64](https://crates.io/crates/base64): For encoding the public key in Base64.
- [Tempfile](https://crates.io/crates/tempfile) (dev dependency): For creating temporary files in tests.

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repository.
3. Run `cargo build` to compile the project.

```bash
$ cargo install --git https://github.com/gofmanaa/ton_pub_key_to_base64
$ ton_pub_key_to_base64 -p liteserver.pub
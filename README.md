# Cargo cleaner

This is a simple script to clean up the `target` directory of a Rust project. It is useful when you want to clean up the `target` directory of a project that you are not currently working on.

The script will search for all `target` directories in the current directory and its subdirectories and delete them. It using the `cargo clean` command to do this.

## Usage

```bash
cargo-cleaner
```
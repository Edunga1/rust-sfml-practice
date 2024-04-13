# Introduction

My first Rust application using SFML.

## Requirements

for macOS. need rust and SFML. Install them with Homebrew.

`brew install rust` and `brew install sfml`

## Run application

`cargo run` to build and run application.

`cargo build` to build and `./target/debug/sfml-rust-start` to run application.

if `cargo run` error with `error while loading shared libraries` then pass environment variable `LD_LIBRARY_PATH` like `LD_LIBRARY_PATH=/usr/local/lib cargo run`

## Build(WIP)

Need to install `cross` to build for other platforms: `cargo install cross --git https://github.com/cross-rs/cross`

then run `cross build --target aarch64-unknown-linux-gnu` to build for Windows.

but it doesn't generate a binary file for Windows now.

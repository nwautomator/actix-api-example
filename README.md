# Actix API example

[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](
https://github.com/magnet/metered-rs)
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-lightgray.svg)](
https://www.rust-lang.org)

## Minimal Actix API example

This repository is meant to demonstrate a minimal working API using the [Actix-web](https://actix.rs/) framework. It also makes use of the excellent [tracing-bunyan-formatter](https://crates.io/crates/tracing-bunyan-formatter) crate to provide structured logging output.

To run this, first install the [Rust compiler](https://rustup.rs/). Next, clone this repository, and simply use `cargo run` to compile the code and boot the API up. You can then play around with the endpoints:

* Static HTML content: [http://localhost:8000/](http://localhost:8000/)
* Current time of day: [http://localhost:8000/time](http://localhost:8000/time)
* Random Lorem ipsum quote generator: [http://localhost:8000/quote](http://localhost:8000/quote)

Of course this barely scratches the surface of what Actix-web is capable of. I recommend reading the [documentation](https://actix.rs/docs) to learn more!
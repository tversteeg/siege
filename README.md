[![Build Status](https://travis-ci.org/tversteeg/siege.svg?branch=master)](https://travis-ci.org/tversteeg/siege)

# siege-editor

[![Cargo](https://img.shields.io/crates/v/siege-editor.svg)](https://crates.io/crates/siege-editor) [![License: GPL-3.0](https://img.shields.io/crates/l/siege-editor.svg)](#license) [![Downloads](https://img.shields.io/crates/d/siege-editor.svg)](#downloads)

## Run

On Linux you need the `xorg-dev` package as required by `minifb` -- `sudo apt install xorg-dev`

    cargo run --release

# siege (Library)

A Rust library for procedurally rendering siege engines.

[![Cargo](https://img.shields.io/crates/v/siege.svg)](https://crates.io/crates/siege) [![License: GPL-3.0](https://img.shields.io/crates/l/siege.svg)](#license) [![Downloads](https://img.shields.io/crates/d/siege.svg)](#downloads)

### [Documentation](https://docs.rs/siege/)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
siege = "0.1"
```

And this to your crate root:

```rust
extern crate siege;
```

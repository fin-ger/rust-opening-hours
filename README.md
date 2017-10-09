# Opening Hours Library for Rust
[![crates.io](https://img.shields.io/crates/v/opening-hours.svg)](https://crates.io/crates/opening-hours)
[![Latest Tag](https://img.shields.io/github/tag/fin-ger/rust-opening-hours.svg)](https://github.com/fin-ger/rust-opening-hours/releases)
[![Build Status](https://travis-ci.org/fin-ger/rust-opening-hours.svg?branch=master)](https://travis-ci.org/fin-ger/rust-opening-hours)
[![Documentation](https://docs.rs/opening-hours/badge.svg)](https://docs.rs/opening-hours/)
[![Homepage](https://img.shields.io/badge/github.io-homepage-blue.svg)](https://fin-ger.github.io/rust-opening-hours/)

Store opening hours of a service or place.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
opening_hours = "^0"
```

Or, if you want [Serde](https://github.com/serde-rs/serde) support, include itlike this:

```toml
[dependencies]
opening_hours = { version = "^0", features = ["with-serde"] }
```

Then put this in your crate root:

```rust
extern crate opening_hours;
```

## How to Run the Examples

In order to run an example from the `example` folder issue the following command.

```
$ cargo run --example <name>
```

## License

This project is licensed under the GPL-v3 license - see the [LICENSE](LICENSE) file for details.

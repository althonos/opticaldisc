# `opticaldisc`

*Read optical media filesystems with Rust .*

[![TravisCI](https://img.shields.io/travis/althonos/opticaldisc/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/opticaldisc/branches)
[![Codecov](https://img.shields.io/codecov/c/github/althonos/opticaldisc.svg?maxAge=600&style=flat-square)](https://codecov.io/github/althonos/opticaldisc)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/opticaldisc)
[![Crate](https://img.shields.io/crates/v/opticaldisc.svg?maxAge=600&style=flat-square)](https://crates.io/crates/opticaldisc)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/opticaldisc)
[![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=2678400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](http://keepachangelog.com/)
[![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=2678400&style=flat-square)](https://saythanks.io/to/althonos)


## Dependencies

| Package | Description | Minimum | Latest  | Source | License |
| --- | --- | --- | --- | --- | --- |
| **nom** | byte parser combinators | ![4.0.0](https://img.shields.io/badge/crates.io-v4.0.0-blue.svg?style=flat-square&maxAge=2678400) | [![latest](https://img.shields.io/crates/v/nom.svg?style=flat-square&maxAge=600)](https://crates.io/crates/nom) | [![GitHub](https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=2678400)](https://github.com/Geal/nom)   | [![MIT](https://img.shields.io/badge/license-MIT/Unlicense-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/unlicense/) |
| **memchr** | safe interface to `memchr` | ![2.0.0](https://img.shields.io/badge/crates.io-v2.0.0-blue.svg?style=flat-square&maxAge=2678400) | [![latest](https://img.shields.io/crates/v/memchr.svg?style=flat-square&maxAge=600)](https://crates.io/crates/memchr) | [![GitHub](https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=2678400)](https://github.com/BurntSushi/rust-memchr)   | [![MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/) |
| **error-chain** | convenient errors management | ![0.11.0](https://img.shields.io/badge/crates.io-v0.11.0-orange.svg?style=flat-square&maxAge=2678400) | [![latest](https://img.shields.io/crates/v/error-chain.svg?style=flat-square&maxAge=600)](https://crates.io/crates/error-chain) | [![GitHub](https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=2678400)](https://github.com/rust-lang-nursery/error-chain) | [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT/Apache_2.0-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/apache-2.0/) |
| **btoi** | convert strings to ints | ![0.3.0](https://img.shields.io/badge/crates.io-v0.3.0-orange.svg?style=flat-square&maxAge=2678400) | [![latest](https://img.shields.io/crates/v/btoi.svg?style=flat-square&maxAge=600)](https://crates.io/crates/btoi) | [![GitHub](https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=2678400)](https://github.com/niklasf/rust-btoi) | [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT/Apache_2.0-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/apache-2.0/) |
| **chrono** | date and time management | ![0.4.0](https://img.shields.io/badge/crates.io-v0.4.0-orange.svg?style=flat-square&maxAge=2678400) | [![latest](https://img.shields.io/crates/v/chrono.svg?style=flat-square&maxAge=600)](https://crates.io/crates/chrono) | [![GitHub](https://img.shields.io/badge/source-GitHub-303030.svg?style=flat-square&maxAge=2678400)](https://github.com/chronotope/chrono) | [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT/Apache_2.0-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/apache-2.0/)


## Quickstart

Add this crate to your `Cargo.toml` manifest:

```toml
[dependencies]
opticaldisc = "^0.1.0"
```


## Usage

### Open an ISO-9660 filesystem

Open an ISO filesystem from anything that's both `Read` and `Seek`:
```rust
extern crate opticaldisc;

let file: std::fs::File = ...;
let iso = opticaldisc::iso::IsoFs::new(file)
```

It's also possible to read a buffer containing binary data (using
[`std::io::Cursor`](https://doc.rust-lang.org/std/io/struct.Cursor.html)
to emulate a file).
```rust
extern crate opticaldisc;

let data = include_bytes!("...");
let iso = opticaldisc::iso::IsoFs::from_buffer(&data[..]);
```

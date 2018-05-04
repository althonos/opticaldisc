//! Read optical media filesystems with Rust.
//!
//! [![TravisCI](https://img.shields.io/travis/althonos/opticaldisc/master.svg?maxAge=600&style=flat-square)](https://travis-ci.org/althonos/opticaldisc/branches)
//! [![Codecov](https://img.shields.io/codecov/c/github/althonos/opticaldisc.svg?maxAge=600&style=flat-square)](https://codecov.io/github/althonos/opticaldisc)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
//! [![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/althonos/opticaldisc)
//! [![Crate](https://img.shields.io/crates/v/opticaldisc.svg?maxAge=86400&style=flat-square)](https://crates.io/crates/opticaldisc)
//! [![Documentation](https://img.shields.io/badge/docs-latest-4d76ae.svg?maxAge=2678400&style=flat-square)](https://docs.rs/opticaldisc)
//! [![CargoMake](https://img.shields.io/badge/built%20with-cargo--make-yellow.svg?maxAge=2678400&style=flat-square)](https://sagiegurari.github.io/cargo-make)
//! [![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](http://keepachangelog.com/)
//! [![SayThanks](https://img.shields.io/badge/say-thanks!-1EAEDB.svg?maxAge=2678400&style=flat-square)](https://saythanks.io/to/althonos)


#![allow(unused_variables)]
#![allow(dead_code)]

extern crate btoi;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate memchr;
#[macro_use]
extern crate nom;

#[macro_use]
mod utils;

pub mod error;
pub mod iso;

pub use self::error::Result;
pub use self::error::Error;
pub use self::error::ErrorKind;

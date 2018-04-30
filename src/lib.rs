//! A crate to parse filesystems used on optical media storage.
//!
//! # Filesystems
//!
//! The following filesystems can be parsed by `opticaldisc`:
//!
//! * [ISO-9660](https://en.wikipedia.org/wiki/ISO_9660)
//!

#![feature(slice_patterns)]
#![feature(crate_in_paths)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate btoi;
extern crate chrono;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

#[macro_use]
mod utils;

pub mod error;
pub mod iso;

pub use self::error::Result;
pub use self::error::Error;
pub use self::error::ErrorKind;

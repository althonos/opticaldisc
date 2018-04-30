//! A crate to parse filesystems used on optical media storage.
#![feature(slice_patterns)]
#![feature(crate_in_paths)]
#![feature(transpose_result)]
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

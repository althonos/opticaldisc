//! A crate to parse filesystems used on optical media storage.
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

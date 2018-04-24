#![feature(slice_patterns)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate btoi;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate fixedvec;
#[macro_use]
extern crate nom;

#[macro_use]
mod utils;

pub mod error;
pub mod datetime;
pub mod iso;

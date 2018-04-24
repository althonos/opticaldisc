mod parser;

use datetime::Datetime;
use utils::parsers::{both_u16, both_u32};

use super::super::record::Record;

#[derive(Debug)]
pub struct PrimaryVolumeDescriptor {
    pub root: Record,
}

impl PrimaryVolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> ::error::Result<Self> {
        match parser::pvd(bytes) {
            Ok((_, pvd)) => Ok(pvd),
            Err(err) => Err(err.into()),
        }
    }
}

pub mod parser;

use chrono::DateTime;
use chrono::offset::FixedOffset;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub date: DateTime<FixedOffset>,
    pub name: String,
    pub extent: u32,
    pub ear_length: u8,
    pub data_length: u32,
    pub seq_number: u16,
    pub version: Option<u8>,
    pub is_dir: bool,
    pub is_hidden: bool,
}

impl Record {
    pub fn parse(input: &[u8]) -> crate::error::Result<Self> {
        Ok(parser::record(input)?.1)
    }
}

pub mod parser;
mod children;

use datetime::Datetime;

use super::image::IsoImage;
use self::children::Children;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub date: Datetime,
    pub name: String,
    pub extent: u32,
    pub extent_length: u8,
    pub data_length: u32,
    pub seq_number: u16,
}

impl Record {
    pub fn parse(input: &[u8]) -> ::error::Result<Self> {
        Ok(parser::record(input)?.1)
    }

    pub fn children<'a, H: 'a>(&'a self, image: &'a mut IsoImage<H>) -> Children<'a, H>
    where
        H: ::std::io::Seek + ::std::io::Read,
    {
        Children::new(self, image)
    }
}

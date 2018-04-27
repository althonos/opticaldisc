mod parser;
mod children;

use chrono::DateTime;
use chrono::offset::FixedOffset;

use error::{Error, ErrorKind, Result};
use super::IsoImage;
use self::children::Children;


#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub date: DateTime<FixedOffset>,
    pub name: String,
    pub extent: u32,
    pub ear_length: u8,
    pub data_length: u32,
    pub seq_number: u16,
    pub version: Option<u8>,
    pub dir: bool,
    pub hidden: bool,
}

impl Record {

    pub(crate) fn parse(input: &[u8]) -> ::error::Result<Self> {
        Ok(parser::record(input)?.1)
    }

    pub(crate) fn children<'a, H: 'a>(
        &'a self,
        image: &'a IsoImage<H>,
    ) -> Result<Children<'a, H>>
    where
        H: ::std::io::Seek + ::std::io::Read,
    {
        if self.dir {
            Ok(Children::new(self, image))
        } else {
            Err(Error::from_kind(ErrorKind::DirectoryExpected))
        }
    }

}

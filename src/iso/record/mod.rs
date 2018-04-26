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

    version: Option<u8>,
    _dir: bool,
    _hidden: bool,
}

impl Record {
    pub(super) fn parse(input: &[u8]) -> ::error::Result<Self> {
        Ok(parser::record(input)?.1)
    }

    pub(super) fn children<'a, H: 'a>(
        &'a self,
        image: &'a mut IsoImage<H>,
    ) -> Result<Children<'a, H>>
    where
        H: ::std::io::Seek + ::std::io::Read,
    {
        if self.is_dir() {
            Ok(Children::new(self, image))
        } else {
            Err(Error::from_kind(ErrorKind::DirectoryExpected))
        }
    }

    pub fn is_dir(&self) -> bool {
        self._dir
    }

    pub fn is_file(&self) -> bool {
        !self._dir
    }
}

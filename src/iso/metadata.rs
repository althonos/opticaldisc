use std::convert::From;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::rc::Rc;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

use super::node::Node;
use super::readdir::ReadDir;
use super::IsoFs;

/// Metadata information about an ISO-9600 filesystem resource.
///
/// Similar to [`std::fs::Metadata`]
pub struct Metadata(Rc<Node>);

impl Metadata {

    pub fn is_dir(&self) -> bool {
        self.0.as_ref().record.is_dir
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn name(&self) -> &str {
        self.0.as_ref().record.name.as_ref()
    }

    pub fn path(&self) -> &Path {
        &self.0.as_ref().path
    }

    pub fn read_dir<H: Seek + Read>(&self, iso: &mut IsoFs<H>) -> Result<ReadDir> {
        if self.is_dir() {
            self.0.load_children(&mut iso.handle)?;
            ReadDir::new(self.0.clone())
        } else {
            Err(Error::from_kind(ErrorKind::DirectoryExpected))
        }
    }
    
}

#[doc(hidden)]
impl From<Rc<Node>> for Metadata {
    fn from(noderc: Rc<Node>) -> Self {
        Metadata(noderc)
    }
}

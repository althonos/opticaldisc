use std::convert::From;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::rc::Rc;

use super::super::error::Error;
use super::super::error::ErrorKind;
use super::super::error::Result;

use super::node::Node;
use super::readdir::ReadDir;
use super::IsoFs;

/// Metadata information about an ISO-9600 filesystem resource.
///
/// Similar to [`std::fs::Metadata`].
///
/// [`std::fs::Metadata`](https://doc.rust-lang.org/std/fs/struct.Metadata.html)
pub struct Metadata(Rc<Node>);

impl Metadata {
    /// Returns whether this metadata is for a directory.
    pub fn is_dir(&self) -> bool {
        self.0.as_ref().record.is_dir
    }

    /// Returns whether this metadata is for a regular file.
    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    /// Return the name of the resource this metadata is for.
    pub fn name(&self) -> &str {
        self.0.as_ref().record.name.as_ref()
    }

    /// Return the absolute path to the resource this metadata is for.
    pub fn path(&self) -> &Path {
        &self.0.as_ref().path
    }

    /// Given an ISO-9660 filesystem, return the contents of this directory.
    ///
    /// This method can be used to implement recursive functions using metadata
    /// objects instead of plain paths.
    ///
    /// # Example
    ///
    /// Use `read_dir` to count the number of files and directories in an
    //  ISO image recursively:
    ///
    /// ```rust
    /// use opticaldisc::iso::{IsoFs, Metadata};
    /// # use std::io::{Seek, Read};
    ///
    /// fn count<H: Seek + Read>(iso: &mut IsoFs<H>, meta: &Metadata) -> usize {
    ///     meta.read_dir(iso)
    ///         .map(|rd| rd.into_iter()
    ///                     .fold(1, |acc, child| acc + count(iso, &child)))
    ///         .unwrap_or(1)
    /// }
    ///
    /// # let path = "static/iso/alpine.level1.iso";
    /// # let mut iso = opticaldisc::iso::IsoFs::from_path(path).unwrap();
    /// let root = iso.metadata("/").unwrap();
    /// # assert_eq!(
    /// count(&mut iso, &root)
    /// # , 125);
    /// ```
    ///
    /// # Warning
    ///
    /// Do not use this function with an `IsoFs` this `Metadata` instance was
    /// not obtained from ! You'll likely receive a nonsensical result, but
    /// this could possibly cause the internal parser to panic.
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

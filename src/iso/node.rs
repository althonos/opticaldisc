use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

use super::record::Record;
use super::constants::SECTOR_SIZE;

/// A node from an ISO-9660 filesystem.
pub(in iso) struct Node {
    pub path: PathBuf,
    pub record: Record,
    pub contents: RefCell<Option<HashMap<String, Rc<Node>>>>,
}

impl Node {
    /// Create a new root node from the given record (used in PVD).
    pub(in iso) fn create_root(record: Record) -> Self {
        Self {
            path: PathBuf::from("/"),
            record: record,
            contents: RefCell::new(None),
        }
    }

    /// Create a child node from the given record (to add to the contents map).
    fn create_child(&self, record: Record) -> Self {
        Self {
            path: self.path.join(&record.name),
            record: record,
            contents: RefCell::new(None),
        }
    }

    /// Find the child of given `name`, using `handle` to parse contents of
    /// directories that are yet unknown.
    pub(in iso) fn child<H>(&self, name: &str, handle: &mut H) -> Result<Rc<Self>>
    where
        H: Read + Seek,
    {
        if !self.record.is_dir {
            bail!(ErrorKind::DirectoryExpected)
        }
        self.load_children(handle)?;
        match self.contents.borrow().as_ref().unwrap().get(name) {
            Some(rc) => Ok(rc.clone()),
            None => Err(Error::from(ErrorKind::NotFound(self.path.join(name)))),
        }
    }

    /// Load the children directory records if they are still unknown.
    ///
    /// Expects `self` to be a directory, or bad things could occur.
    pub(in iso) fn load_children<H>(&self, handle: &mut H) -> Result<()>
    where
        H: Read + Seek,
    {
        if self.contents.borrow().is_none() {
            self.parse_children(handle)
        } else {
            Ok(())
        }
    }

    /// Parse the children records of `self`.
    ///
    /// Expects `self` to be a directory, or bad things could occur.
    fn parse_children<H>(&self, handle: &mut H) -> Result<()>
    where
        H: Read + Seek,
    {
        use super::record::parser::record;

        let mut total: u64 = 0;
        let mut offset: usize;
        let mut contents = HashMap::new();
        let mut buffer = [255; SECTOR_SIZE as usize];

        // seek to the initial extent position
        handle.seek(SeekFrom::Start(self.record.extent as u64 * SECTOR_SIZE))?;

        // a sector with directory records starts with the length of the first
        // directory record, which can never have a length of 0
        'sector: while total < self.record.data_length as u64 {
            // read the next sector and reset the buffer offset
            offset = 0;
            handle.read_exact(&mut buffer)?;
            total += SECTOR_SIZE;
            // parse records while there are more records to be found
            'record: while buffer[offset] != 0 {
                // parse the next record and advance the buffer cursor
                let (rem, record) = record(&buffer[offset..])?;
                offset = SECTOR_SIZE as usize - rem.len();

                // check the record is not another directory, and add it to
                // the directory contents hashmap
                if record.name == "\0" && record.extent != self.record.extent {
                    break 'sector;
                } else if record.name != "\0" && record.name != "\x01" {
                    let name = record.name.clone();
                    let node = Rc::new(self.create_child(record));
                    println!("ADDED {}", &name);
                    contents.insert(name, node);
                }
            }
        }

        // replace the directory contents with the parsed children
        self.contents.replace(Some(contents));
        Ok(())
    }
}

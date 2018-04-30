use std::borrow::Borrow;
use std::cell::RefCell;
use std::cell::RefMut;
use std::collections::HashMap;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

use super::record::Record;
use super::readdir::ReadDir;
use super::constants::SECTOR_SIZE;

/// A node from an ISO-9660 filesystem.
pub(in iso) struct Node {
    pub path: PathBuf,
    pub record: Record,
    pub contents: RefCell<Option<HashMap<String, Rc<Node>>>>,
}

impl Node {
    pub(in iso) fn create_root(record: Record) -> Self {
        Self {
            path: PathBuf::from("/"),
            record: record,
            contents: RefCell::new(None),
        }
    }

    pub(in iso) fn create_child(&self, record: Record) -> Self {
        Self {
            path: self.path.join(&record.name),
            record: record,
            contents: RefCell::new(None),
        }
    }
}

impl Node {
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

    fn parse_children<H>(&self, handle: &mut H) -> Result<()>
    where
        H: Read + Seek,
    {
        let mut offset: usize;
        let mut contents = HashMap::new();
        let mut buffer = [0; SECTOR_SIZE as usize];

        handle.seek(SeekFrom::Start(self.record.extent as u64 * SECTOR_SIZE))?;

        loop {
            offset = 0;
            handle.read_exact(&mut buffer)?;

            while let Ok((rem, record)) = super::record::parser::record(&buffer[offset..]) {
                offset = SECTOR_SIZE as usize - rem.len();
                if record.name == "\0" && record.extent != self.record.extent {
                    self.contents.replace(Some(contents));
                    return Ok(());
                } else if record.name != "\0" && record.name != "\x01" {
                    let name = record.name.clone();
                    let node = Rc::new(self.create_child(record));
                    contents.insert(name, node);
                }
            }
        }
    }
}

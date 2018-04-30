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
use super::constants::SECTOR_SIZE;

/// A node from an ISO-9660 filesystem.
pub(in iso) struct Node {
    path: PathBuf,
    record: Record,
    contents: Option<HashMap<String, Rc<RefCell<Node>>>>,
}

impl Node {
    pub(in iso) fn create_root(record: Record) -> Self {
        Self {
            path: PathBuf::from("/"),
            record: record,
            contents: None,
        }
    }

    pub(in iso) fn create_child(&self, record: Record) -> Self {
        Self {
            path: self.path.join(&record.name),
            record: record,
            contents: None,
        }
    }
}

impl Node {
    pub(in iso) fn is_dir(&self) -> bool {
        self.record.is_dir
    }

    pub(in iso) fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub(in iso) fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub(in iso) fn child<H>(&mut self, name: &str, handle: &RefCell<H>) -> Result<Rc<RefCell<Self>>>
    where
        H: Read + Seek,
    {
        if self.is_file() {
            bail!(ErrorKind::DirectoryExpected)
        }

        if self.contents.is_none() {
            self.parse_children(handle)?;
        }

        self.contents
            .as_ref()
            .unwrap()
            .get(name)
            .map(|rc| rc.clone())
            .ok_or(Error::from(ErrorKind::NotFound(self.path.join(name))))
    }

    fn parse_children<H>(&mut self, handle: &RefCell<H>) -> Result<()>
    where
        H: Read + Seek,
    {
        let mut offset: usize;
        let mut contents = HashMap::new();
        let mut buffer = [0; SECTOR_SIZE as usize];

        handle
            .borrow_mut()
            .seek(SeekFrom::Start(self.record.extent as u64 * SECTOR_SIZE));

        loop {
            offset = 0;
            handle.borrow_mut().read_exact(&mut buffer)?;

            while let Ok((rem, record)) = super::record::parser::record(&buffer[offset..]) {
                offset = SECTOR_SIZE as usize - rem.len();
                if record.name == "\0" && record.extent != self.record.extent {
                    self.contents = Some(contents);
                    return Ok(());
                } else if record.name != "\0" && record.name != "\x01" {
                    let name = record.name.clone();
                    let node = Rc::new(RefCell::new(self.create_child(record)));
                    contents.insert(name, node);
                }
            }
        }
    }
}

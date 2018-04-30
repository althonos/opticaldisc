//! ISO-9660 parser library.

mod descriptors;
mod file;
mod metadata;
mod node;
mod readdir;
mod record;

mod constants {
    pub const SECTOR_SIZE: u64 = 2048;
    pub const DEFAULT_BLOCK_SIZE: u64 = 2048;
}

pub use self::file::IsoFile;
pub use self::readdir::ReadDir;
pub use self::metadata::Metadata;

use std::cell::RefCell;
use std::cell::RefMut;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use super::error::Result;
use super::error::Error;
use super::error::ErrorKind;

use self::node::Node;
use self::record::Record;

/// An ISO-9660 filesystem.
pub struct IsoFs<H: Read + Seek> {
    handle: RefCell<H>,
    root: Rc<RefCell<Node>>,
    block_size: u16,
}

// Common methods
impl<H: Read + Seek> IsoFs<H> {
    /// Get an iterator over a directory content.
    pub fn read_dir<P: AsRef<Path>>(&self) -> Result<ReadDir> {
        panic!("TODO")
    }

    /// Get metadata about a resource located at the given path.
    pub fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        self.metadata_impl(path.as_ref()).map(Metadata::from)
    }

    fn metadata_impl(&self, path: &Path) -> Result<Rc<RefCell<Node>>> {
        let mut node: Rc<RefCell<Node>> = self.root.clone();

        for component in path.components() {
            use std::path::Component::*;
            node = match component {
                Prefix(_) => bail!(ErrorKind::Msg(String::from("what the fuck are you doing?"))),
                CurDir => node,
                RootDir => self.root.clone(),
                Normal(name) => {
                    let name_str = name.to_str().expect("not utf-8");
                    node.as_ref().borrow_mut().child(name_str, &self.handle)?
                }
                ParentDir => {
                    self.metadata_impl(node.as_ref().borrow().path().parent().expect("no parent"))?
                }
            }
        }

        Ok(node)
    }

    /// Check if a directory exists on the filesystem.
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path)
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
    }

    /// Check if a file exists on the filesystem.
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.metadata(path)
            .map(|meta| meta.is_file())
            .unwrap_or(false)
    }

    /// Check if a resource with the given path exists on the filesystem.
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        panic!("TODO")
    }

    /// Open the file located
    pub fn open_file<P: AsRef<Path>>(&mut self, p: P) -> Result<IsoFile<H>> {
        panic!("TODO")
    }
}

// Constructor
impl<H: Read + Seek> IsoFs<H> {
    /// Open an `IsoFs` stored in the given handle.
    pub fn new(mut handle: H) -> Result<Self> {
        use self::descriptors::VolumeDescriptor;

        let mut block_size: Option<u16> = None;
        let mut root: Option<Node> = None;

        let mut descriptors: Vec<VolumeDescriptor> = Vec::new();
        let mut offset = 0x10;
        let mut buff = [0; self::constants::SECTOR_SIZE as usize];
        let mut terminated = false;

        // Go to the 16th logical sector
        handle.seek(::std::io::SeekFrom::Start(
            offset * self::constants::SECTOR_SIZE,
        ))?;

        // Read all volume descriptors and extract data from the PVD
        while let (false, Ok(vd)) = (
            terminated,
            handle
                .read_exact(&mut buff)
                .map_err(Error::from)
                .and_then(|_| VolumeDescriptor::parse(&buff)),
        ) {
            offset += 1;
            if let VolumeDescriptor::Terminator(_) = vd {
                terminated = true
            } else if let VolumeDescriptor::Primary(ref pvd) = vd {
                block_size = Some(pvd.block_size);
                root = Some(Node::create_root(pvd.root.clone()))
            }

            descriptors.push(vd);
        }

        // Assert the loop did not break because of an error
        if !terminated {
            bail!(ErrorKind::NoPrimaryVolumeDescriptor);
        }

        Ok(Self {
            handle: RefCell::new(handle),
            block_size: block_size.ok_or(ErrorKind::NoPrimaryVolumeDescriptor)?,
            root: root.ok_or(ErrorKind::NoPrimaryVolumeDescriptor)
                .map(RefCell::new)
                .map(Rc::new)?,
        })
    }
}

// Constructor from file
impl IsoFs<::std::fs::File> {
    /// Open an `IsoFs` located on the filesystem at the given path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        ::std::fs::File::open(path.as_ref())
            .map_err(Error::from)
            .and_then(Self::new)
    }
}

// Constructor from byte buffer
impl<B: AsRef<[u8]>> IsoFs<::std::io::Cursor<B>> {
    /// Open an `IsoFs` contained in a buffer of bytes.
    pub fn from_buffer(buffer: B) -> Result<Self> {
        Self::new(::std::io::Cursor::new(buffer))
    }
}

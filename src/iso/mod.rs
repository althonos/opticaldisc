//! [`ISO-9660`] filesystem parser and reader.
//!
//! ISO filesystems are commonly found on optical medias such as CD-ROMs, but can also be obtained
//! from archive files (known as *iso images*) which are 1:1 dumps of ISO-9660 formatted storage.
//! ISO images are the primary distribution format of many Linux distributions.
//!
//! # Parser
//!
//! The `iso` module uses the [`nom`](https://docs.rs/nom/) crate to parse a file-handle containing
//! an ISO filesystem. Directory contents are discovered lazily, and only the *root* directory is
//! loaded in memory when creating a new [`IsoFs`].
//!
//! # References
//!
//! Since it cannot be known whether a directory was parsed already or not, most of the methods
//! of [`IsoFs`] will take a *mutable* reference [`&mut self`] instead of a constant reference
//! [`&self`]. This makes use of the Rust rule enforcing only a single mutable reference to an
//! object at a time, which is used here to protect the internal file-handle from concurrent
//! access, all done at compile-time by the borrow checker.
//!
//! If you need to share multiple references to an IsoImage, you should use a [`RefCell`].
//!
//! # Examples
//!
//! Open a the `static/iso/alpine.level1.iso` file and find all the directories in the *root*:
//!
//! ```rust
//! # extern crate opticaldisc;
//! use opticaldisc::iso::Metadata;
//!
//! let mut iso = opticaldisc::iso::IsoFs::from_path("static/iso/alpine.level1.iso").unwrap();
//! let contents: Vec<Metadata> = iso.read_dir("/").unwrap().into_iter().collect();
//! # assert!(!contents.is_empty())
//! ```
//!

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
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::rc::Rc;

use super::error::Result;
use super::error::Error;
use super::error::ErrorKind;

use self::node::Node;

/// An ISO-9660 filesystem.
pub struct IsoFs<H: Read + Seek> {
    handle: H,
    root: Rc<Node>,
    block_size: u16,
}

// Common methods
impl<H: Read + Seek> IsoFs<H> {
    /// Get an iterator over a directory content.
    ///
    /// The directory contents are loaded before the [`ReadDir`] iterator is created if they were
    /// not already. This allows the iterator to outlive the reference to the `IsoFs`.
    ///
    /// # Errors
    ///
    /// * [`NotFound`] when the resource could not be found
    /// * [`DirectoryExpected`] when the resource is not a directory
    ///
    /// # Example
    ///
    /// ```rust
    /// # use std::path::Path;
    /// # let mut iso = opticaldisc::iso::IsoFs::from_path("static/iso/alpine.level1.iso").unwrap();
    /// for entry in iso.read_dir("ETC/APK").unwrap().into_iter() {
    ///    if entry.name() == "ARCH" {
    ///        assert!(entry.is_file());
    ///        assert_eq!(entry.path(), Path::new("/ETC/APK/ARCH"));
    ///    }
    /// }
    /// ```
    pub fn read_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<ReadDir> {
        let node = self.node(path.as_ref())?;
        node.as_ref().load_children(&mut self.handle)?;
        ReadDir::new(node)
    }

    /// Get metadata about a resource located at the given path.
    pub fn metadata<P: AsRef<Path>>(&mut self, path: P) -> Result<Metadata> {
        self.node(path.as_ref()).map(Metadata::from)
    }

    fn node(&mut self, path: &Path) -> Result<Rc<Node>> {
        let mut node: Rc<Node> = self.root.clone();

        for component in path.components() {
            use std::path::Component::*;
            node = match component {
                Prefix(_) => bail!(ErrorKind::Msg(String::from("what the fuck are you doing?"))),
                CurDir => node,
                RootDir => self.root.clone(),
                Normal(name) => {
                    let name_str = name.to_str().expect("not utf-8");
                    node.as_ref().child(name_str, &mut self.handle)?
                }
                ParentDir => self.node(node.as_ref().path.parent().expect("no parent"))?,
            }
        }

        Ok(node)
    }

    /// Check if the given path maps to a directory on the filesystem.
    pub fn is_dir<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.node(path.as_ref())
            .map(|n| n.as_ref().record.is_dir)
            .unwrap_or(false)
    }

    /// Check if the given path maps to a file on the filesystem.
    pub fn is_file<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.node(path.as_ref())
            .map(|n| !n.as_ref().record.is_dir)
            .unwrap_or(false)
    }

    /// Check if a resource with the given path exists on the filesystem.
    pub fn exists<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.node(path.as_ref()).is_ok()
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
            handle,
            block_size: block_size.ok_or(ErrorKind::NoPrimaryVolumeDescriptor)?,
            root: match root {
                Some(node) => Rc::new(node),
                None => bail!(ErrorKind::NoPrimaryVolumeDescriptor),
            },
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

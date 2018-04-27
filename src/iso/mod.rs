mod descriptors;
mod file;
mod record;

pub use self::file::File;
pub use self::record::Record;

use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use super::error::{Error, ErrorKind, Result};
use self::descriptors::VolumeDescriptor;
use self::descriptors::PrimaryVolumeDescriptor;

/// The size of a *sector* on the ISO.
///
/// Not that it is **not** the *logical block* size, which is defined in the Primary
/// Volume Descriptor, although they are equal most of the time.
const SECTOR_SIZE: usize = 2048;

/// An ISO-9660 filesystem.
#[derive(Debug)]
pub struct IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    handle: H,
    descriptors: Vec<VolumeDescriptor>,
    records: HashMap<PathBuf, Rc<Record>>,
    block_size: usize,
}

impl<H> IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    /// Open an `IsoImage` stored in the given handle.
    pub fn new(mut handle: H) -> Result<Self> {

        let mut block_size = None;
        let mut records = HashMap::new();
        let mut descriptors = Vec::new();
        let mut offset = 0x10;
        let mut buff = [0; SECTOR_SIZE];
        let mut terminated = false;

        while let (false, Ok(ref bytes)) =
            (terminated, Self::get_sector(&mut handle, offset, &mut buff))
        {
            let vd = VolumeDescriptor::parse(bytes)?;
            offset += 1;

            match vd {
                VolumeDescriptor::Terminator(_) => terminated = true,
                VolumeDescriptor::Primary(ref pvd) => {
                    block_size = Some(pvd.block_size as usize);
                    records.insert(PathBuf::from("/"), Rc::clone(&pvd.root));
                }
                _ => (),
            }

            descriptors.push(vd);
        }

        if !terminated {
            bail!(::error::ErrorKind::NoSetTerminator)
        }

        Ok(Self {
            handle,
            descriptors,
            records,
            block_size: block_size.ok_or(ErrorKind::NoPrimaryVolumeDescriptor)?,
        })
    }

    /// Load a sector into the given buffer.
    ///
    /// Buffer must have a capacity of exactly `SECTOR_SIZE`.
    fn get_sector<'a>(handle: &mut H, block: u32, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let offset = (block * SECTOR_SIZE as u32) as u64;
        handle.seek(::std::io::SeekFrom::Start(offset))?;
        handle.read_exact(buf)?;
        Ok(buf)
    }

    /// Load a logical block into the given buffer.
    ///
    /// Buffer must have a capacity of exactly `self.block_size`.
    fn get_block<'a>(&mut self, block: u32, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let offset = (block * self.block_size as u32) as u64;
        self.handle.seek(::std::io::SeekFrom::Start(offset))?;
        self.handle.read_exact(buf)?;
        Ok(buf)
    }

    /// Get the record for the given path, or `Error::NotFound`.
    fn get_record<'a>(&'a mut self, path: &::std::path::Path) -> Result<Rc<Record>> {

        // {
        //     let records = &self.records;
        if let Some(record) = self.records.get(path) {
            return Ok(record.clone());
        }

        if let Some(parent_path) = path.parent() {

            let filename = path.file_name();
            let parent = self.get_record(parent_path)?;

            for child in parent.children(self)?.collect::<Vec<Record>>() {
                self.records.insert(parent_path.join(&child.name), Rc::new(child));
            }

            match self.records.get(path) {
                Some(record) => Ok(record.clone()),
                None => Err(Error::from_kind(ErrorKind::NotFound(path.to_path_buf())))
            }

        } else {
            Ok(self.pvd().root.clone())
        }

    }

    /// Get the Primary Volume Descriptor of this image.
    pub fn pvd<'a>(&'a self) -> &'a PrimaryVolumeDescriptor {
        self.descriptors
            .iter()
            .filter_map(|x| match x {
                VolumeDescriptor::Primary(pvd) => Some(pvd),
                _ => None,
            })
            .next()
            .unwrap()
    }

    pub fn read_dir<'a, P>(&'a mut self, path: P) -> Result<Vec<Record>>
    where
        P: AsRef<::std::path::Path>,
    {
        let ref record = self.get_record(path.as_ref())?;
        Ok(record.children(self)?.collect())
    }

    pub fn open_file<'a, P>(&'a mut self, path: P) -> Result<File<'a, H>>
    where
        P: AsRef<::std::path::Path>,
    {
        let record = self.get_record(path.as_ref())?;
        Ok(File::new(
            &mut self.handle,
            record.extent * self.block_size as u32,
            record.data_length,
        ))
    }
}

impl IsoImage<::std::fs::File> {
    /// Open an `IsoImage` located on the filesystem at the given path.
    pub fn from_path<P>(path: P) -> Result<Self>
    where
        P: ::std::convert::AsRef<::std::path::Path>,
    {
        ::std::fs::File::open(path.as_ref())
            .map_err(Error::from)
            .and_then(Self::new)
    }
}

impl<B> IsoImage<::std::io::Cursor<B>>
where
    B: ::std::convert::AsRef<[u8]>,
{
    /// Open an `IsoImage` containted in a buffer of bytes.
    pub fn from_buffer(buffer: B) -> Result<Self> {
        Self::new(::std::io::Cursor::new(buffer))
    }
}

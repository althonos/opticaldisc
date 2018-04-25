mod descriptor;
mod file;
mod record;

pub use self::record::Record;
pub use self::file::File;

use std::path::Path;
use super::error::{Error, ErrorKind, Result};
use self::descriptor::VolumeDescriptor;
use self::descriptor::PrimaryVolumeDescriptor;


/// A disk image.
#[derive(Debug)]
pub struct IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    handle: H,
    descriptors: Vec<VolumeDescriptor>,
    blocksize: u32,
}

impl<H> IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn get_block<'a>(&mut self, block: u32, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let offset = (block * self.blocksize) as u64;
        self.handle.seek(::std::io::SeekFrom::Start(offset))?;
        self.handle.read_exact(buf)?;
        Ok(buf)
    }

    pub fn pvd(&self) -> Result<&PrimaryVolumeDescriptor> {
        self.descriptors
            .iter()
            .filter_map(|x| match x {
                VolumeDescriptor::Primary(pvd) => Some(pvd),
                _ => None,
            })
            .next()
            .ok_or(Error::from_kind(ErrorKind::NoPrimaryVolumeDescriptor))
    }

    pub fn get_record<P>(&mut self, path: P) -> Result<Record>
    where
        P: AsRef<::std::path::Path>,
    {
        let ref _path = path.as_ref();
        let root = self.pvd()?.root.clone();
        let mut record = root;

        for component in _path.components() {
            use std::path::Component;
            record = match component {
                Component::Normal(s) => record
                    .children(self)?
                    .find(|r| s == r.name.as_str())
                    .ok_or(Error::from_kind(ErrorKind::NotFound(_path.to_path_buf())))?,
                Component::RootDir => record,
                _ => record,
            };
        }

        Ok(record)
    }

    pub fn list_records<P>(&mut self, path: P) -> Result<Vec<Record>>
    where
        P: AsRef<::std::path::Path>,
    {
        let record = self.get_record(path)?;
        Ok(record.children(self)?.collect())
    }

    pub fn listdir<P>(&mut self, path: P) -> Result<Vec<String>>
    where
        P: AsRef<::std::path::Path>,
    {
        let record = self.get_record(path)?;
        println!("{:?}", record);
        Ok(record.children(self)?.map(|r| r.name).collect())
    }

    pub fn open_file<'a, P>(&'a mut self, path: P) -> Result<File<'a, H>>
    where
        P: AsRef<::std::path::Path>,
    {
        let record = self.get_record(path)?;
        Ok(File::new(&mut self.handle, record.extent * self.blocksize, record.data_length))
    }
}

//
impl<H> IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn new(handle: H) -> Result<Self> {
        let mut iso = Self {
            handle,
            descriptors: Vec::new(),
            blocksize: 2048,
        };

        let mut offset = 0x10;
        let mut buff = vec![0; iso.blocksize as usize];

        while let Ok(ref bytes) = iso.get_block(offset, &mut buff) {
            match VolumeDescriptor::parse(bytes)? {
                vd @ VolumeDescriptor::Terminator(_) => {
                    iso.descriptors.push(vd);
                    return Ok(iso);
                }
                vd => {
                    iso.descriptors.push(vd);
                    offset += 1;
                }
            }
        }

        bail!(::error::ErrorKind::NoSetTerminator)
    }
}

impl IsoImage<::std::fs::File> {
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
    pub fn from_buffer(buffer: B) -> Result<Self> {
        Self::new(::std::io::Cursor::new(buffer))
    }
}

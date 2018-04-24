use error::{Error, ErrorKind, Result};

use super::descriptor::VolumeDescriptor;
use super::descriptor::PrimaryVolumeDescriptor;
use super::record::Record;

const LOGICAL_BLOCK_SIZE: usize = 2048;

/// A disk image.
#[derive(Debug)]
pub struct IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub handle: H,
    pub descriptors: Vec<VolumeDescriptor>,
}

impl<H> IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn get_block<'a>(&mut self, block: u32, buf: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let offset = (block * LOGICAL_BLOCK_SIZE as u32) as u64;
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
}

impl<H> IsoImage<H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn open_file(handle: H) -> Result<Self> {
        let mut iso = Self {
            handle,
            descriptors: Vec::new(),
        };

        let mut offset = 0x10;
        let mut buff = [0; 2048];

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
                    .children(self)
                    .find(|r| s == r.name.as_str())
                    .ok_or(Error::from_kind(ErrorKind::NotFound(_path.to_path_buf())))?,
                Component::RootDir => record,
                _ => record,
            };
        }

        Ok(record)
    }

    pub fn listdir<P>(&mut self, path: P) -> Result<Vec<Record>>
    where
        P: AsRef<::std::path::Path>,
    {
        let record = self.get_record(path)?;
        Ok(record.children(self).collect())
    }
}

impl IsoImage<::std::fs::File> {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: ::std::convert::AsRef<::std::path::Path>,
    {
        ::std::fs::File::open(path)
            .map_err(Error::from)
            .and_then(Self::open_file)
    }
}

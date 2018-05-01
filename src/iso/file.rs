use std::io::SeekFrom;
use std::io::Result;

/// Readable file located on an ISO-9660 filesystem.
pub struct IsoFile<'a, H: 'a>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    handle: &'a mut H,
    start: u32,
    length: u32,
    pos: u64,
}

impl<'a, H: 'a> IsoFile<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn new(handle: &'a mut H, start: u32, length: u32) -> Result<Self> {
        match handle.seek(SeekFrom::Start(start as u64)) {
            Ok(_) => Ok(Self {
                handle,
                start,
                length,
                pos: 0,
            }),
            Err(err) => Err(err),
        }
    }
}

impl<'a, H: 'a> ::std::io::Read for IsoFile<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let size = self.length as u64 - self.pos;
        let bytes_read = self.handle.read(&mut buffer[..size as usize])?;
        self.pos += bytes_read as u64;
        Ok(bytes_read)
    }
}

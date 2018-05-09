use std::cmp::min;
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
        handle.seek(SeekFrom::Start(start as u64))?;
        Ok(Self {
            handle,
            start,
            length,
            pos: 0,
        })
    }
}

impl<'a, H: 'a> ::std::io::Read for IsoFile<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let size = min(self.length as usize - self.pos as usize, buffer.len());
        let bytes_read = self.handle.read(&mut buffer[..size])?;
        self.pos += bytes_read as u64;
        Ok(bytes_read)
    }
}

impl<'a, H: 'a> ::std::io::Seek for IsoFile<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    fn seek(&mut self, whence: SeekFrom) -> Result<u64> {
        use std::io::Error;
        use std::io::ErrorKind::InvalidInput;

        let err = Error::new(InvalidInput, "invalid seek to a negative position");

        self.pos = match whence {
            SeekFrom::Current(x) if self.pos as i64 + x < 0 => return Err(err),
            SeekFrom::Current(x) => min(self.pos + x as u64, self.length as u64),
            SeekFrom::End(x) if self.length as i64 + x < 0 => return Err(err),
            SeekFrom::End(x) => min(self.length as i64 + x, self.length as i64) as u64,
            SeekFrom::Start(x) => min(x, self.length as u64),
        };

        Ok(self.pos)
    }
}

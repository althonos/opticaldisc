pub struct File<'a, H: 'a>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    handle: &'a mut H,
    start: u32,
    length: u32,
    pos: u64,
}

impl<'a, H: 'a> File<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn new(handle: &'a mut H, start: u32, length: u32) -> Self {
        handle.seek(::std::io::SeekFrom::Start(start as u64));
        Self {
            handle,
            start,
            length,
            pos: 0,
        }
    }
}

impl<'a, H: 'a> ::std::io::Read for File<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    fn read(&mut self, buffer: &mut [u8]) -> ::std::io::Result<usize> {
        let res = self.handle.take(self.length as u64 - self.pos).read(buffer);
        if let Ok(bytes_read) = res {
            self.pos += bytes_read as u64
        };
        res
    }
}

use super::Record;
use super::IsoImage;

pub struct Children<'a, H: 'a>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    parent: &'a Record,
    image: &'a mut IsoImage<H>,
    offset: usize,
    block: u32,
    buffer: Vec<u8>,
    finished: bool,
}

impl<'a, H: 'a> Children<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    pub fn new(parent: &'a Record, image: &'a mut IsoImage<H>) -> Self {
        Children {
            parent: parent,
            buffer: vec![0; image.block_size],
            image,
            block: parent.extent,
            offset: 0,
            finished: false,
        }
    }
}

impl<'a, H: 'a> ::std::iter::Iterator for Children<'a, H>
where
    H: ::std::io::Seek + ::std::io::Read,
{
    type Item = Record;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished {
            while let Ok((rem, record)) = super::parser::record(&self.buffer[self.offset..]) {
                self.offset = self.buffer.len() - rem.len();
                if record.name == "\0" && record.extent != self.parent.extent {
                    self.finished = true;
                    break;
                } else if record.name != "\0" && record.name != "\x01" {
                    return Some(record);
                }
            }

            if !self.finished {
                self.finished = self.image.get_block(self.block, &mut self.buffer).is_err();
                self.block += 1;
                self.offset = 0;
            }
        }

        None
    }
}

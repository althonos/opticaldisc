mod boot;
mod terminator;
mod primary;

pub use self::boot::BootRecord;
pub use self::terminator::SetTerminator;
pub use self::primary::PrimaryVolumeDescriptor;

use nom::Err::Incomplete;
use nom::Needed::Size;

use super::constants::SECTOR_SIZE;

#[derive(Debug)]
pub enum VolumeDescriptor {
    Boot(BootRecord),
    Terminator(SetTerminator),
    Primary(PrimaryVolumeDescriptor),
}

impl VolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> ::error::Result<Self> {
        use self::VolumeDescriptor::*;

        Ok(match bytes.first() {
            None => bail!(Incomplete::<&[u8]>(Size(SECTOR_SIZE as usize))),
            Some(&0x00) => Boot(BootRecord::parse(bytes)?),
            Some(&0x01) => Primary(PrimaryVolumeDescriptor::parse(bytes)?),
            Some(&0xFF) => Terminator(SetTerminator::parse(bytes)?),
            Some(&othr) => bail!(::error::ErrorKind::UnknownDescriptorType(othr)),
        })
    }
}

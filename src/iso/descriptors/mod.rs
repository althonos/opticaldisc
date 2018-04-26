mod boot;
mod terminator;
mod primary;

use error::Result;
use error::ErrorKind;

pub use self::boot::BootRecord;
pub use self::terminator::SetTerminator;
pub use self::primary::PrimaryVolumeDescriptor;

#[derive(Debug)]
pub enum VolumeDescriptor {
    Boot(BootRecord),
    Terminator(SetTerminator),
    Primary(PrimaryVolumeDescriptor),
}

impl VolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        Ok(match bytes.first() {
            None => bail!(::nom::Err::Incomplete::<&[u8]>(::nom::Needed::Size(2048))),
            Some(0x00) => VolumeDescriptor::Boot(BootRecord::parse(bytes)?),
            Some(0x01) => VolumeDescriptor::Primary(PrimaryVolumeDescriptor::parse(bytes)?),
            Some(0xFF) => VolumeDescriptor::Terminator(SetTerminator::parse(bytes)?),
            Some(othr) => bail!(ErrorKind::UnknownDescriptorType(*othr)),
        })
    }
}

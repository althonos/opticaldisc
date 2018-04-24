mod parser;

#[derive(Debug)]
pub enum BootSystemUse {
    Unknown,
}

#[derive(Debug)]
pub struct BootRecord {
    version: u8,
    system_identifier: String,
    boot_identifier: String,
    system_use: BootSystemUse,
}

impl BootRecord {
    pub fn parse<'a>(bytes: &[u8]) -> ::error::Result<Self> {
        match parser::boot_record(bytes) {
            Ok((_, record)) => Ok(record),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::iter::FromIterator;
    use super::BootRecord;

    #[test]
    fn test_parse() {
        let mut buf = Vec::new();
        buf.extend(b"\x00CD001\x01EL TORITO SPECIFICATION");
        buf.extend(&vec![0; 2050]);
        let record = BootRecord::parse(&buf).unwrap();
        assert_eq!(record.version, 1);
        assert_eq!(record.system_identifier, "EL TORITO SPECIFICATION");
    }

}

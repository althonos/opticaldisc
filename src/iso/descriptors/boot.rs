#[derive(Debug)]
pub struct BootRecord {
    version: u8,
    system_identifier: String,
    boot_identifier: String,
    system_use: Vec<u8>,
}

impl BootRecord {
    pub fn parse<'a>(bytes: &[u8]) -> ::error::Result<Self> {
        match parser::boot_record(bytes) {
            Ok((_, record)) => Ok(record),
            Err(err) => Err(err.into()),
        }
    }
}

mod parser {

    use nom::be_u8;
    use super::BootRecord;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    named!(pub boot_record(&[u8]) -> BootRecord,
        do_parse!(
                     tag!(b"\0")                                           >>
                     tag!(b"CD001")                                        >>
            version: be_u8                                                 >>
            sys_id:  map_res!(null_terminated!(32), ::std::str::from_utf8) >>
            boot_id: map_res!(null_terminated!(32), ::std::str::from_utf8) >>
            sys_use: take!(1977)                                           >>
                     (BootRecord {
                         version,
                         system_identifier: sys_id.to_owned(),
                         boot_identifier: boot_id.to_owned(),
                         system_use: sys_use.to_vec(),
                     })
        )
    );

    #[cfg(test)]
    mod tests {

        use std::iter::FromIterator;
        use super::BootRecord;

        #[test]
        fn test_boot_record() {
            let mut buf = Vec::new();
            buf.extend(b"\x00CD001\x01EL TORITO SPECIFICATION");
            buf.extend(&vec![0; 2050]);
            let (remaining, record) = super::boot_record(&buf).unwrap();
            assert_eq!(remaining, &buf[2048..]);
            assert_eq!(record.version, 1);
            assert_eq!(record.system_identifier, "EL TORITO SPECIFICATION");
        }
    }

}

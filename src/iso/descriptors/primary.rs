use super::super::record::Record;

#[derive(Debug)]
pub struct PrimaryVolumeDescriptor {
    pub root: Record,
    pub block_size: u16,
}

impl PrimaryVolumeDescriptor {
    pub fn parse(bytes: &[u8]) -> ::error::Result<Self> {
        match parser::pvd(bytes) {
            Ok((_, pvd)) => Ok(pvd),
            Err(err) => Err(err.into()),
        }
    }
}

mod parser {

    use btoi::btou;

    use nom::be_u8;

    use chrono::DateTime;
    use chrono::TimeZone;
    use chrono::offset::FixedOffset;

    use utils::parsers::both_u16;
    use utils::parsers::both_u32;

    use super::PrimaryVolumeDescriptor;
    use super::Record;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    named!(datetime(&[u8]) -> DateTime<FixedOffset>,
        // TODO: finer-grained parser
        // FIXME: no chrono panic
        do_parse!(
            year:   map_res!(take!(4), btou::<i32>) >>
            month:  map_res!(take!(2), btou::<u32>) >>
            day:    map_res!(take!(2), btou::<u32>) >>
            hour:   map_res!(take!(2), btou::<u32>) >>
            min:    map_res!(take!(2), btou::<u32>) >>
            sec:    map_res!(take!(2), btou::<u32>) >>
            hun:    map_res!(take!(2), btou::<u32>) >>
            tz:     be_u8                    >>
                    (
                        FixedOffset::east((tz as i32 - 48) * 900)
                            .ymd(year, month, day)
                            .and_hms_milli(hour, min, sec, hun*10)
                    )
        )
    );

    #[cfg_attr(rustfmt, rustfmt_skip)]
    named!(pub pvd(&[u8]) -> PrimaryVolumeDescriptor,
        do_parse!(
                            tag!(b"\x01")                      >>
                            tag!(b"CD001")                     >>
            version:        tag!(b"\x01")                      >>
                            tag!(b"\x00")                      >>
            system_id:      take!(32)                          >>
            volume_id:      take!(32)                          >>
                            take!(8)                           >>
            space_size:     both_u32                           >>
                            take!(32)                          >>
            set_size:       both_u16                           >>
            seq_number:     both_u16                           >>
            block_size:     both_u16                           >>
            pt_size:        both_u32                           >>
                            take!(16)                          >>
            root:           map_res!(take!(34), Record::parse) >>
            set_id:         take!(128)                         >>
            pub_id:         take!(128)                         >>
            prep_id:        take!(128)                         >>
            app_id:         take!(128)                         >>
            copyr_file:     take!(38)                          >>
            abstract_file:  take!(36)                          >>
            biblio_file:    take!(37)                          >>
            creattime:      datetime                           >>
            modifstime:     datetime                           >>

            // TODO:        optional dates (all zeros for None)
            expirtime:      take!(17)                          >>
            effectime:      take!(17)                          >>

            fs_version:     tag!(b"\x01")                      >>
                            tag!(b"\0")                        >>
            app_used:       take!(512)                         >>
            reserved:       take!(653)                         >>
                            (PrimaryVolumeDescriptor {
                                root,
                                block_size
                            })
        )
    );

    #[cfg(test)]
    mod tests {

        use chrono::Datelike;
        use chrono::Timelike;

        #[test]
        fn test_datetime() {
            let buf1 = b"1996111316301203\x0A";
            let (_, dt) = super::datetime(buf1).unwrap();
            assert_eq!(dt.year(), 1996);
            assert_eq!(dt.month(), 11);
            assert_eq!(dt.day(), 13);
            assert_eq!(dt.hour(), 16);
            assert_eq!(dt.minute(), 30);
            assert_eq!(dt.second(), 12);
            assert_eq!(dt.nanosecond(), 30_000_000);
            // assert_eq!(dt.tz, 0x0A);
        }
    }

}

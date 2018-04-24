use nom::be_u8;
use utils::parsers::{both_u16, both_u32};
use datetime::Datetime;

use super::super::super::record::Record;
use super::PrimaryVolumeDescriptor;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(datetime(&[u8]) -> Datetime,
    do_parse!(
        year:   map_res!(take!(4), ::btoi::btoi) >>
        month:  map_res!(take!(2), ::btoi::btoi) >>
        day:    map_res!(take!(2), ::btoi::btoi) >>
        hour:   map_res!(take!(2), ::btoi::btoi) >>
        min:    map_res!(take!(2), ::btoi::btoi) >>
        sec:    map_res!(take!(2), ::btoi::btoi) >>
        hun:    map_res!(take!(2), ::btoi::btoi) >>
        tz:     be_u8                            >>
                (Datetime {
                    year,
                    month,
                    day,
                    hour,
                    minute: min,
                    second: sec,
                    hundredth: hun,
                    tz
                })
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
        expirtime:      datetime                           >>
        effectime:      datetime                           >>
        fs_version:     tag!(b"\x01")                      >>
                        tag!(b"\0")                        >>
        app_used:       take!(512)                         >>
        reserved:       take!(653)                         >>
                        (PrimaryVolumeDescriptor {
                            root
                        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_datetime() {
        let buf1 = b"1996111316301203\x0A";
        let (_, dt) = super::datetime(buf1).unwrap();
        assert_eq!(dt.year, 1996);
        assert_eq!(dt.month, 11);
        assert_eq!(dt.day, 13);
        assert_eq!(dt.hour, 16);
        assert_eq!(dt.minute, 30);
        assert_eq!(dt.second, 12);
        assert_eq!(dt.hundredth, 03);
        assert_eq!(dt.tz, 0x0A);
    }
}

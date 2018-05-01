use btoi::btou;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::offset::FixedOffset;

use memchr::memrchr;

use nom::be_u8;
use nom::Context;
use nom::Err::Failure;

use utils::parsers::both_u16;
use utils::parsers::both_u32;

use super::Record;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(datetime(&[u8]) -> DateTime<FixedOffset>,
    do_parse!(
        year:  be_u8 >>
        month: matching!(1...12) >>
        day:   matching!(1...31) >>
        hour:  matching!(0...23) >>
        min:   matching!(0...59) >>
        sec:   matching!(0...59) >>
        tz:    matching!(0...100) >>
               (FixedOffset::east((tz as i32 - 48) * 900)
                    .ymd(year as i32 + 1900, month as u32, day as u32)
                    .and_hms(hour as u32, min as u32, sec as u32)
               )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn versioned_name(input: &[u8], is_dir: bool) -> ::nom::IResult<&[u8], (&str, Option<u8>)> {
    let version: Option<u8>;
    let name: &[u8];
    let (i1, size) = try_parse!(input, be_u8);
    let (i2, buff) = try_parse!(i1, take!(size));
    let len = size as usize;

    if let Some(sep) = memrchr(b';', buff) {
        name = if buff[sep - 1] == b'.' { &buff[..sep - 1] } else { &buff[..sep] };
        version = match btou(&buff[sep+1..]) {
            Ok(version_num) => Some(version_num),
            Err(_) => return Err(Failure(Context::Code(&buff[sep+1..], ::nom::ErrorKind::MapRes))),
        };
    } else {
        name = &buff[..];
        version = None
    }

    match ::std::str::from_utf8(name) {
        Ok(name_str) => Ok((i2, (name_str, version))),
        Err(_) => Err(Failure(Context::Code(&buff[len-1..], ::nom::ErrorKind::MapRes))),
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(record_flags(&[u8]) -> (bool, bool, bool, bool, bool, bool),
    bits!(
        do_parse!(
            extend: take_bits!(u8, 1)   >>
                    take_bits!(u8, 2)   >>
            perms:  take_bits!(u8, 1)   >>
            info:   take_bits!(u8, 1)   >>
            assoc:  take_bits!(u8, 1)   >>
            isdir:  take_bits!(u8, 1)   >>
            hidden: take_bits!(u8, 1)   >>
                    (
                        hidden == 1,
                         isdir == 1,
                         assoc == 1,
                          info == 1,
                         perms == 1,
                        extend == 1
                    )
        )
    )
);

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub record(&[u8]) -> Record,
    do_parse!(
                        not!(peek!(tag!("\0")))         >>
        length:         be_u8                           >>
        ear_length:     be_u8                           >>
        extent:         both_u32                        >>
        data_length:    both_u32                        >>
        date:           datetime                        >>
        flags:          record_flags                    >>
        unit_size:      be_u8                           >>
        gap_size:       be_u8                           >>
        seq_number:     both_u16                        >>
        id_length:      peek!(be_u8)                    >>
        versioned_id:   apply!(versioned_name, flags.1) >>
                        take!(length - id_length - 33)  >>
                        (Record {
                            name: versioned_id.0.to_owned(),
                            version: versioned_id.1,
                            date,
                            extent,
                            ear_length,
                            data_length,
                            seq_number,
                            is_hidden: flags.0,
                            is_dir: flags.1
                        })
    )
);

#[cfg(test)]
mod tests {

    use chrono::Datelike;
    use chrono::Timelike;

    #[test]
    fn test_filename() {
        let buf = b"\x06APK.;1";
        let (_, (name, v)) = super::versioned_name(buf, false).unwrap();
        assert_eq!(v, Some(1));
        assert_eq!(name, "APK");
    }

    #[test]
    fn test_record() {
        let buf = b"\"\0\x13\0\0\0\0\0\0\x13\0\x08\0\0\0\0\x08\0v\x04\x01\x05\x05\x17\0\x02\0\0\x01\0\0\x01\x01\0";
        let (_, dr) = super::record(buf).unwrap();

        let buf2 = b"`\x00\x13\x00\x00\x00\x00\x00\x00\x13\x00\x08\x00\x00\x00\x00\x08\x00v\x04\x01\x05\x05\x17\x00\x02\x00\x00\x01\x00\x00\x01\x01\x01PX$\x01\xedA\x00\x00\x00\x00A\xed\x01\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00TF\x1a\x01\x0ev\x04\x01\x05\x05\x17\x00v\x04\x01\x05\x05\x18\x00v\x04\x01\x05\x05\x17\x00";
        let (_, dr) = super::record(&buf2[..]).unwrap();

        let buf3 = b"\x28\x00\xf6\x00\x00\x00\x00\x00\x00\xf6\x88\x16\x03\x00\x00\x03\x16\x88v\x05\x01\x0f3\x1e\x08\x00\x00\x00\x01\x00\x00\x01\x06APK.;1\x00";
        let (_, dr) = super::record(&buf3[..]).unwrap();
        assert_eq!(dr.name, "APK");

        let buf4 = b"\0\0\0\0\0";
        assert!(super::record(&buf4[..]).is_err());
    }

    #[test]
    fn test_datetime() {
        let buf = b"\x76\x0B\x0D\x09\x23\x2D\x01";
        let (_, dt) = super::datetime(buf).unwrap();
        assert_eq!(dt.year(), 2018);
        assert_eq!(dt.month(), 11);
        assert_eq!(dt.day(), 13);
        assert_eq!(dt.hour(), 9);
        assert_eq!(dt.minute(), 35);
        assert_eq!(dt.second(), 45);
    }

}

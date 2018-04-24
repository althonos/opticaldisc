use utils::parsers::{both_u16, both_u32};
use datetime::Datetime;

use super::Record;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn datetime(input: &[u8]) -> ::nom::IResult<&[u8], Datetime> {
    use ::nom::be_u8;
    do_parse!(input,
        year:  be_u8 >>
        month: be_u8 >>
        day:   be_u8 >>
        hour:  be_u8 >>
        min:   be_u8 >>
        sec:   be_u8 >>
        tz:    be_u8 >>
               (Datetime {
                    year: (year as u16) + 1900 ,
                    month,
                    day,
                    hour,
                    minute: min,
                    second: sec,
                    hundredth: 0,
                    tz
                })
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn record(input: &[u8]) -> ::nom::IResult<&[u8], Record> {
    use ::nom::be_u8;
    let (_, length) = peek!(input, be_u8)?;
    let (rem, buf) = take!(input, length)?;
    do_parse!(buf,
        length:         be_u8                                                        >>
        extent_length:  be_u8                                                        >>
        extent:         both_u32                                                     >>
        data_length:    both_u32                                                     >>
        date:           datetime                                                     >>
        flags:          take!(1)                                                     >>
        unit_size:      be_u8                                                        >>
        gap_size:       be_u8                                                        >>
        seq_number:     both_u16                                                     >>
        id_length:      be_u8                                                        >>
        id:             map_res!(take!(id_length), ::std::str::from_utf8) >>
                        (Record {
                            name: id.to_owned(),
                            date,
                            extent,
                            extent_length,
                            data_length,
                            seq_number,
                        })
    ).map(|(_, r)| (rem, r))
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_record() {
        let buf = b"\"\0\x13\0\0\0\0\0\0\x13\0\x08\0\0\0\0\x08\0v\x04\x01\x05\x05\x17\0\x02\0\0\x01\0\0\x01\x01\0";
        let dr = super::record(buf).unwrap();

        let buf2 = b"`\x00\x13\x00\x00\x00\x00\x00\x00\x13\x00\x08\x00\x00\x00\x00\x08\x00v\x04\x01\x05\x05\x17\x00\x02\x00\x00\x01\x00\x00\x01\x01\x01PX$\x01\xedA\x00\x00\x00\x00A\xed\x01\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00TF\x1a\x01\x0ev\x04\x01\x05\x05\x17\x00v\x04\x01\x05\x05\x18\x00v\x04\x01\x05\x05\x17\x00";
        let dr = super::record(&buf2[..]).unwrap();
    }

    #[test]
    fn test_datetime() {
        let buf = b"\x76\x0B\x0D\x09\x23\x2D\x01";
        let (_, dt) = super::datetime(buf).unwrap();
        assert_eq!(dt.year, 2018);
        assert_eq!(dt.month, 11);
        assert_eq!(dt.day, 13);
        assert_eq!(dt.hour, 9);
        assert_eq!(dt.minute, 35);
        assert_eq!(dt.second, 45);
        assert_eq!(dt.hundredth, 0);
        assert_eq!(dt.tz, 1);
    }

}

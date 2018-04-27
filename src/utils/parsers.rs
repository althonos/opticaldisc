/// Parse a null-terminated string from a fixed-length field.
macro_rules! null_terminated (
    ($i: expr, $size: expr) => ({
        take!($i, $size)
            .and_then(|(rem, raw)| take_while!(raw, |x| x != 0).map(|(_, s)| (rem, s)))
    })
);

/// Parse an u8 only if it matches the given pattern.
macro_rules! matching {
    ($i: expr, $pattern: pat) => ({
        match $i.get(0) {
            None => Err(::nom::Err::Incomplete(::nom::Needed::Size(1))),
            Some(x @ $pattern) => Ok((&$i[1..], *x)),
            _ => Err(::nom::Err::Error(::nom::Context::Code($i, ::nom::ErrorKind::Custom(0)))),
        }
    })
}

/// Common code generation for both-endian platform-dependent parser.
macro_rules! both_endian_impl {
    ($name: ident, $type: ty, $le: path, $be: path) => {
        #[cfg(target_endian = "little")]
        pub fn $name(input: &[u8]) -> ::nom::IResult<&[u8], $type> {
            terminated!(
                input,
                $le,
                take!($crate::std::mem::size_of::<$type>())
            )
        }
        #[cfg(target_endian = "big")]
        pub fn $name(input: &[u8]) -> ::nom::IResult<&[u8], $type> {
            preceded!(
                input,
                take!($crate::std::mem::size_of::<$type>())
                $be,
            )
        }
    }
}

both_endian_impl!(both_u16, u16, ::nom::le_u16, ::nom::be_u16);
both_endian_impl!(both_u32, u32, ::nom::le_u32, ::nom::be_u32);
both_endian_impl!(both_u64, u64, ::nom::le_u64, ::nom::be_u64);


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_null_terminated() {
        let buf1 = b"TEST\0\0\0";
        assert_eq!(null_terminated!(&buf1[..], 6), Ok((&buf1[6..], &buf1[..4])));
    }

    #[test]
    fn test_matching() {
        let buf1 = b"\x02";
        assert!(matching!(&buf1[..], _).is_ok());
        assert!(matching!(&buf1[..], 0).is_err());
        assert!(matching!(&buf1[..], 1...5).is_ok());
        assert!(matching!(&buf1[..], 1...2).is_ok());
    }

    #[test]
    fn test_both_u16() {
        let buf1 = b"\x01\x00\x00\x01";
        assert_eq!(both_u16(buf1), Ok((&buf1[4..], 0x0001)));
        let buf2 = b"\x01\x02\x02\x01";
        assert_eq!(both_u16(buf2), Ok((&buf2[4..], 0x0201)));
    }

    #[test]
    fn test_both_u32() {
        let buf1 = b"\x01\0\0\0\0\0\0\x01";
        assert_eq!(both_u32(buf1), Ok((&buf1[8..], 0x00000001)));
        let buf2 = b"\x01\x02\x03\x04\x04\x03\x02\x01";
        assert_eq!(both_u32(buf2), Ok((&buf2[8..], 0x04030201)));
    }
}

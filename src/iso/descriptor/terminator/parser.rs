use nom::be_u8;
use super::SetTerminator;

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(pub terminator(&[u8]) -> SetTerminator,
    do_parse!(
                 tag!(b"\xFF")  >>
                 tag!(b"CD001") >>
        version: be_u8          >>
                 take!(2041)    >>
                 (SetTerminator {
                     version
                 })
    )
);

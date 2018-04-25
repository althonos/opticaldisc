error_chain!{
    foreign_links {
        Io(::std::io::Error);
    }
    errors {
        // DirectoryExpected(path: ::std::path::PathBuf) {
        DirectoryExpected {
            description("directory expected")
            display("directory expected")
        }
        FileExpected {
            description("file expected")
            display("file expected")
        }
        NoPrimaryVolumeDescriptor {
            description("no primary volume descriptor found")
            display("no primary volume descriptor found")
        }
        NotFound(path: ::std::path::PathBuf) {
            description("path not found")
            display("path not found: '{}'", path.to_string_lossy())
        }
        NoSetTerminator {
            description("missing set terminator")
            display("missing set terminator")
        }
        ParseError(kind: ::nom::ErrorKind) {
            description("parse error")
            display("parse error: {}", kind.description())
        }
        ParseIncomplete(needed: Option<usize>) {
            description("not enough data")
            display("not enough data")
        }
        UnknownDescriptorType(t: u8) {
            description("unknown descriptor type")
            display("unknown descriptor type: {}", t)
        }
    }
}

impl<E: ::std::fmt::Debug + Clone> ::std::convert::From<::nom::Err<E>> for Error {
    fn from(err: ::nom::Err<E>) -> Self {
        use nom::Err::Incomplete;
        use nom::Needed;

        match err {
            Incomplete(Needed::Unknown) => ErrorKind::ParseIncomplete(None),
            Incomplete(Needed::Size(x)) => ErrorKind::ParseIncomplete(Some(x)),
            other => ErrorKind::ParseError(other.into_error_kind()),
        }.into()
    }
}

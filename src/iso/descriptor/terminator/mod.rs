mod parser;

#[derive(Debug)]
pub struct SetTerminator {
    version: u8,
}

impl SetTerminator {
    pub fn parse(bytes: &[u8]) -> ::error::Result<Self> {
        match parser::terminator(bytes) {
            Ok((_, term)) => Ok(term),
            Err(err) => Err(err.into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::iter::FromIterator;
    use super::SetTerminator;

    #[test]
    fn test_parse() {
        let mut buf = Vec::new();
        buf.extend(b"\xFFCD001\x01");
        buf.extend(&vec![0; 2050]);
        let record = SetTerminator::parse(&buf).unwrap();
        assert_eq!(record.version, 1);
    }

}

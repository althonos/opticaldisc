// TODO:
// - iterate on contents instead of collecting everything (tricky lifetimes)

use std::rc::Rc;

use super::super::error::Error;
use super::super::error::ErrorKind;
use super::super::error::Result;

use super::metadata::Metadata;
use super::node::Node;

pub struct ReadDir {
    node: Rc<Node>,
    entries: Vec<Metadata>,
}

impl ReadDir {
    pub(in iso) fn new(node: Rc<Node>) -> Result<Self> {
        if node.as_ref().record.is_dir {
            let entries = node.as_ref()
                .contents
                .borrow()
                .as_ref()
                .unwrap()
                .values()
                .map(Clone::clone)
                .map(Metadata::from)
                .collect();
            Ok(Self { node, entries })
        } else {
            Err(Error::from_kind(ErrorKind::DirectoryExpected))
        }
    }
}

impl IntoIterator for ReadDir {
    type Item = Metadata;
    type IntoIter = ::std::vec::IntoIter<Metadata>;
    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

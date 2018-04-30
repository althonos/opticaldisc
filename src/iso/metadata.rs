use std::convert::From;
use std::rc::Rc;
use std::cell::RefCell;

use super::node::Node;

/// Metadata information about an ISO-9600 filesystem resource.
///
/// Similar to [`std::fs::Metadata`]
pub struct Metadata(Rc<RefCell<Node>>);

impl Metadata {
    pub fn is_dir(&self) -> bool {
        self.0.as_ref().borrow().is_dir()
    }

    pub fn is_file(&self) -> bool {
        !self.0.as_ref().borrow().is_dir()
    }
}

#[doc(hidden)]
impl From<Rc<RefCell<Node>>> for Metadata {
    fn from(node_ref: Rc<RefCell<Node>>) -> Self {
        Metadata(node_ref)
    }
}

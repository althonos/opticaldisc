use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;

use super::Record;

pub struct Metadata<'a> {
    path: Cow<'a, Path>,
    record: Rc<Record>,
}

impl<'a> Metadata<'a> {

    pub(crate) fn new(path: &'a Path, record: Rc<Record>) -> Self {
        Metadata {
            path: Cow::Borrowed(path),
            record
        }
    }

    pub fn is_dir(&self) -> bool {
        self.record.dir
    }

    pub fn is_file(&self) -> bool {
        !self.record.dir
    }

}

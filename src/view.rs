//! TODO
//!
use std::path::{Path, PathBuf};

/// # Archive
///
pub struct Archive<'a> {
    path: &'a Path,

}

impl <'a> Archive<'a> {

    ///
    pub fn new<P: AsRef<Path>>(path: &'a P) -> Archive<'a> {
        Archive {
            path: path.as_ref(),
        }
    }
}

/// # Entry
pub struct Entry {

}

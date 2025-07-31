//!
//! TODO

use crate::error::ViewError;

pub(crate) type ViewResult<T> = std::result::Result<T, ViewError>;

pub trait ViewOpener {
    fn open_view() -> ViewResult<Archive>;
}

pub trait Viewer {

    fn name() -> &'static str;
}

pub struct Archive {

}

impl Archive {
    
}

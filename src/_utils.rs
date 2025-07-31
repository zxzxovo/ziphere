//! The mod contains a series of helper function
//! that will be used in other mod.

use crate::_error::Error::IOError;
use crate::_error::IsIOError::{OnError, StdIoError, WalkError};
use crate::_error::Result;
use std::path::Path;
use walkdir::WalkDir;

/// Calculate the size of the passing path.
pub(crate) fn calculate_size<P: AsRef<Path>>(path: P) -> Result<u64> {
    let path = path.as_ref();
    if path.is_file() {
        Ok(std::fs::metadata(path)
            .map_err(|e| IOError(StdIoError(e)))?
            .len())
    } else if path.is_dir() {
        let mut total_size = 0u64;
        for entry in WalkDir::new(path) {
            let entry = entry.map_err(|e| IOError(WalkError(e)))?;
            if entry.file_type().is_file() {
                total_size += entry.metadata().map_err(|e| IOError(WalkError(e)))?.len();
            }
        }
        Ok(total_size)
    } else {
        Err(IOError(OnError("Path is not a file".to_string())))
    }
}

pub(crate) fn into_file<P: AsRef<Path>>(path: P) -> Result<std::fs::File> {
    let path = path.as_ref();
    if !path.exists() {
        return Err(IOError(OnError("Path not exist.".to_string())));
    }
    std::fs::File::open(path).map_err(|e| IOError(StdIoError(e)))
}
//! ## Error
//!
//! Here defines the error types that may happen in the crate.
//!

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {

    #[error("{0}")]
    FsError(String),
    
    #[error("Error when compressing/decompressing.")]
    ComdeError(#[from] ComdeError),
    
    #[error("Error when viewing archive.")]
    ViewError(#[from] ViewError),
    
    #[error("Unknown error.")]
    #[warn(dead_code)]
    OtherError(),
}

#[derive(Debug, Error)]
pub enum ComdeError {

    #[error("Error when compressing/decompressing zip: {0}")]
    ZipError(String),

    #[error("Error when compressing/decompressing 7z: {0}")]
    SevenZError(String),

    #[error("Unfinished")]
    Other(#[from] Unimplemented)
}

#[derive(Debug, Error)]
pub enum ViewError {
    
}

#[derive(Debug, Error)]
pub enum Unimplemented {

    #[error("Unimplemented feature")]
    Unfinished
}
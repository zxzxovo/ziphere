//! ## Error
//!
//! Here defines the error types that may happen in the crate.
//!

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    
    #[error("Error when compressing/decompressing.")]
    ComdeError(#[from] ComdeError),
    
    #[error("Error when viewing archive.")]
    ViewError(#[from] ViewError),
    
    #[error("Unknown error.")]
    OtherError(),
}

#[derive(Debug, Error)]
pub enum ComdeError {
    
}

#[derive(Debug, Error)]
pub enum ViewError {
    
}
//! This module defines the error types used throughout the crate.

use thiserror::Error;

/// The errors may happen in the crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to compress: {0}")]
    CompressError(String),

    #[error("Failed to decompress: {0}")]
    DecompressError(DecompError),

    #[error("Unsupported operation.")]
    Unsupported,

    #[error("IO Operation error: {0}")]
    IOError(IsIOError),
}

#[derive(Debug, Error)]
pub enum DecompError {
    #[error("Passwd incorrect: {0}")]
    PasswdIncorrect(String),

    #[error("Passwd needed: {0}")]
    PasswdNeeded(String),

    #[error("Decompress Error: {0}")]
    DecompressErr(String),
}

#[derive(Debug, Error)]
pub enum IsIOError {
    #[error("IO Error: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("IO Error: {0}")]
    WalkError(#[from] walkdir::Error),

    #[error("IO Error: {0}")]
    OnError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

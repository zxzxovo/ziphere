//! This module defines the error types used throughout the crate.

use thiserror::Error;

/// The errors may happen in the crate.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to compress, due to: {0}")]
    CompressError(String),

    #[error("Failed to decompress, due to: {0}")]
    DecompressError(String),

    #[error("Unsupported operation.")]
    Unsupported,
}

pub type Result<T> = std::result::Result<T, Error>;
//! # ZipHere
//!
//! This crate intend to support **easy-to-use** apis
//! for `compressing` or `decompressing` files.
//!

/// Error types.
pub mod _error;

/// Compress and Decompress.
pub mod _comde;

/// Supported _formats and functions;
pub mod _formats;

/// Used utils
mod _utils;

pub use _error::Result;
pub use _formats::Configs;
pub use _formats::Formats;

pub mod prelude {
    pub use crate::_comde::*;
    pub use crate::_formats::*;
    pub use crate::_error::*;
}
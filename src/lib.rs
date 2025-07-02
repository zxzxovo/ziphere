//! # ZipHere
//!
//! This crate intend to support **easy-to-use** apis
//! for `compressing` or `decompressing` files.
//!

/// Error types.
pub mod error;

/// Compress and Decompress.
pub mod comde;

/// Supported formats and functions;
pub mod formats;

/// Used utils
mod utils;

pub use error::Result;
pub use formats::Configs;
pub use formats::Formats;

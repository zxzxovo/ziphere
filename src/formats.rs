//! Supported formats and corresponding functions
//! u
//! and configs for `compress` and `decompress`.

use crate::formats::sevenz::SevenzComde;
use crate::formats::zip::ZipComde;

/// 7z format implementation.
#[cfg(feature = "sevenz")]
pub mod sevenz;

/// zip format implementation.
#[cfg(feature = "zip")]
pub mod zip;

/// The Comde formats builder.
#[derive(Debug)]
pub enum Formats {
    #[cfg(feature = "zip")]
    Zip(ZipComde),

    #[cfg(feature = "sevenz")]
    SevenZ(SevenzComde),
}

impl Formats {
    /// Get a new compressor.
    pub fn new_with(format: &str) -> Formats {
        match format {
            "zip" | "ZIP" | "Zip" => Formats::Zip(ZipComde),
            "7z" | "7Z" | "sevenz" | "sevenZ" | "SevenZ" => Formats::SevenZ(SevenzComde),
            _ => panic!("Unsupported format."),
        }
    }
}

/// Compression level options.
#[derive(Clone, Copy)]
pub enum CompressLevel {
    Fast,
    Balanced,
    Best,
    Custom(u32),
}

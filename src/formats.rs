//! Supported formats and corresponding functions 
//! u
//! and configs for `compress` and `decompress`.

#[cfg(feature = "zip")]
pub mod zip;
#[cfg(feature = "sevenz")]
pub mod sevenz;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Formats {
    Zip,
    SevenZ,
}

/// Compression level options.
#[derive(Clone, Copy)]
pub enum CompressLevel {
    Fast,
    Balanced,
    Best,
    Custom(u32)
}



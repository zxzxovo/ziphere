//! Operation define to compress and decompress.

use crate::Result;
use std::path::Path;
use std::time::Duration;

/// Common configs for compression or decompression.
pub trait ComdeConfig: Sized {
    /// Start building Config.
    fn new() -> Self;

    /// Configure completed and get the result.
    fn build(self) -> Result<Self> {
        Ok(self)
    }
}

/// The formats that support `compress` option should implement.
pub trait Compress {
    type Configs: ComdeConfig;
    fn compress_file<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        config: Self::Configs,
    ) -> Result<CompressStats>;
}

/// The stats that this compression procession have.
pub struct CompressStats {
    pub origin_size: u64,
    pub compressed_size: u64,
    pub time_cost: Duration,
}

impl CompressStats {
    pub(crate) fn new(origin_size: u64, compressed_size: u64, time_cost: Duration) -> Self {
        Self {
            origin_size,
            compressed_size,
            time_cost,
        }
    }
}

/// The formats that support `decompress` option should implement.
pub trait Decompress {
    type Configs: ComdeConfig;
    fn decompress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> Result<DecompressStats>;
}

/// The stats that this decompression procession have.
pub struct DecompressStats {
    pub compressed_size: u64,
    pub decompressed_size: u64,
    pub time_cost: Duration,
}

impl DecompressStats {
    pub(crate) fn new(compressed_size: u64, decompressed_size: u64, time_cost: Duration) -> Self {
        Self {
            compressed_size,
            decompressed_size,
            time_cost,
        }
    }
}

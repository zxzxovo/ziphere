//! Operation define to compress and decompress.

use std::path::PathBuf;
use std::time::Duration;
use crate::{Result};

/// Common configs for compression or decompression.
pub trait ComdeConfig {
    fn new() -> Self;
}


/// The formats that support `compress` option should implement.
pub trait Compress {
    type Configs: ComdeConfig;
    fn compress_file(
        &self,
        input: PathBuf,
        output: PathBuf,
        config: Self::Configs
    ) -> Result<CompressStats>;
}

/// The stats that this compression procession have.
pub struct CompressStats {
    pub origin_size: u64,
    pub compressed_size: u64,
    pub time_cost: Duration
}

impl CompressStats {
    pub(crate) fn new(origin_size: u64, compressed_size: u64, time_cost: Duration) -> Self {
        Self {
            origin_size,
            compressed_size,
            time_cost
        }
    }
}


/// The formats that support `decompress` option should implement.
pub trait Decompress {
    type Configs: ComdeConfig;
    fn decompress_file(
        input: PathBuf,
        output: PathBuf,
        config: Self::Configs
    ) -> Result<DecompressStats>;
}

/// The stats that this decompression procession have.
pub struct DecompressStats {
    pub compressed_size: u64,
    pub decompressed_size: u64,
    pub time_cost: Duration,
}
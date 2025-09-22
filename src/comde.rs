//! Comde Module

use std::time::Duration;

use crate::utils;

/// The status of a compression.
#[derive(Debug)]
pub struct CompressStatus {
    origin_size: u64,
    compressed_size: u64,
    time: Duration,
}

impl CompressStatus {
    pub fn new(origin_size: u64, compressed_size: u64, time: Duration) -> CompressStatus {
        CompressStatus {
            origin_size,
            compressed_size,
            time
        }
    }
}

impl std::fmt::Display for CompressStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (os, cs) = (utils::h_size(self.origin_size), utils::h_size(self.compressed_size));
        write!(f, "\n\tOrigin files size: {}\n\tCompressed archive size: {}\n\tTime  cost: {}", os, cs, self.time.as_secs())
    }
}


/// The status of a decompression.
#[derive(Debug)]
pub struct DecompressStatus {
    compressed_size: u64,
    decompressed_size: u64,
    time: Duration,
}

impl DecompressStatus {
    pub fn new(compressed_size: u64, decompressed_size:u64, time: Duration) -> DecompressStatus {
        DecompressStatus {
            compressed_size,
            decompressed_size,
            time
        }
    }
}

impl std::fmt::Display for DecompressStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (cs, ds) = (utils::h_size(self.compressed_size), utils::h_size(self.decompressed_size));
        write!(f, "\n\tArchive size: {}\n\tDecompressed files size: {}\n\tTime cost: {}s\n", cs, ds, self.time.as_secs())
    }
}
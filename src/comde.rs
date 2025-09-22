//! Comde Module

use std::time::Duration;


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
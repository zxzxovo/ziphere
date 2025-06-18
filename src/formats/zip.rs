//! .zip format support.

#![cfg(feature = "zip")]

use crate::comde;
use crate::comde::{CompressStats, DecompressStats};
use std::path::Path;

pub struct ZipComde;

impl comde::Compress for ZipComde {
    type Configs = ZipConfig;

    fn compress_file<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        config: Self::Configs,
    ) -> crate::Result<CompressStats> {
        todo!()
    }
}

pub struct ZipConfig;

impl comde::ComdeConfig for ZipConfig {
    fn new() -> Self {
        ZipConfig {}
    }
}

pub struct ZipDeConfig;

impl comde::ComdeConfig for ZipDeConfig {
    fn new() -> Self {
        ZipDeConfig {}
    }
}

impl comde::Decompress for ZipComde {
    type Configs = ZipDeConfig;

    fn decompress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> crate::Result<DecompressStats> {
        todo!()
    }
}

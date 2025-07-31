//! Comde Module
//! TODO

use std::path::Path;
use crate::error::{AppError, ComdeError};

pub(crate) type ComdeResult<T> = std::result::Result<T, ComdeError>;

pub trait Comde {
    type Config: ComdeCfg;
    fn compress(
        input: impl AsRef<Path>,
        output: impl AsRef<Path>,
        config: &Self::Config,
    ) -> ComdeResult<CompressStatus>;

    fn decompress(
        input: impl AsRef<Path>,
        output: impl AsRef<Path>,
        config: &Self::Config,
    ) -> ComdeResult<DecompressStatus>;
}

pub trait ComdeCfg {

}

pub struct CompressStatus {
    origin_size: u64,
    compressed_size: u64,
}

pub struct DecompressStatus {

}
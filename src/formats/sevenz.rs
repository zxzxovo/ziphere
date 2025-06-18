//! `.7z` format support.

use std::path::PathBuf;
use sevenz_rust2::SevenZMethodConfiguration;
use crate::comde;
use crate::comde::{CompressStats, DecompressStats};
use crate::error::Error::{CompressError, IOError};
use crate::error::IsIOError::{OnError, StdIoError};
use crate::utils::calculate_size;

pub struct SevenzCompressor;

impl comde::Compress for SevenzCompressor {
    type Configs = SevenzConfigs;

    fn compress_file(
        &self,
        input: PathBuf,
        output: PathBuf,
        config: Self::Configs
    ) -> crate::Result<CompressStats> {
        if !input.exists() {
            return Err(IOError(OnError("Path not exist.".to_string() )))
        }
        
        // Start timing.
        let start_time = std::time::Instant::now();
        let origin_size = calculate_size(&input)?;
        let mut writer = sevenz_rust2::SevenZWriter::create(&output)
            .map_err(|e| CompressError(e.to_string()))?;
        
        writer.set_content_methods(config.methods);
        
        // Do solid_compress if enable.
        if !config.solid_compress {
            writer.push_source_path_non_solid(&input, |_| true)
        } else {
            writer.push_source_path(&input, |_| true)
        }.map_err(|e| CompressError(e.to_string() ))?;
        
        // Write and stop timing.
        writer.finish().map_err(|e| CompressError(e.to_string()))?;
        let compressed_size = std::fs::metadata(output).map_err(|e|
            IOError(StdIoError(e))
        )?.len();
        let duration = start_time.elapsed();
        
        Ok(CompressStats::new(origin_size, compressed_size, duration))
    }
}

/// The compression configuration of 7z.
pub struct SevenzConfigs {
    solid_compress: bool,
    methods: Vec<SevenZMethodConfiguration>
}

impl comde::ComdeConfig for SevenzConfigs {
    fn new() -> SevenzConfigs {
        SevenzConfigs {
            solid_compress: false,
            methods: Vec::new()
        }
    }

    fn build(self) -> crate::Result<Self> {
        Ok(self)
    }
}

impl SevenzConfigs {
    /// Encrypt with AES256.
    pub fn set_password(mut self, password: &str) -> Self {
        self.methods.push(sevenz_rust2::AesEncoderOptions::new(password.into()).into());
        self
    }
    
    /// Call to enable solid compression.
    pub fn enable_solid_compress(mut self) -> Self {
        self.solid_compress = true;
        self
    }

    /// Use LZMA2 with default arguments.
    pub fn use_lzma2(mut self) -> Self {
        let m = sevenz_rust2::lzma::LZMA2Options::default();
        self.methods.push(m.into());
        self
    }


    /// Use LZMA2 with default arguments.
    pub fn use_lzma2_with_preset(mut self, preset: u32) -> Self {
        let m = sevenz_rust2::lzma::LZMA2Options::with_preset(preset);
        self.methods.push(m.into());
        self
    }

    /// Use ZStandard with given compression level.
    #[cfg(feature = "sevenz-zstd")]
    pub fn use_zstd_with_level(mut self, level: u32) -> Self {
        let m = sevenz_rust2::ZStandardOptions::from_level(level);
        self.methods.push(m.into());
        self
    }

}


impl comde::Decompress for SevenzCompressor {
    type Configs = SevenzConfigs;

    fn decompress_file(input: PathBuf, output: PathBuf, config: Self::Configs) -> crate::Result<DecompressStats> {
        todo!()
    }
}
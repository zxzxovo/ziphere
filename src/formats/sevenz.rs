//! `.7z` format support.

#![cfg(feature = "sevenz")]

use crate::comde;
use crate::comde::{CompressStats, DecompressStats};
use crate::error::DecompError;
use crate::error::Error::{CompressError, DecompressError, IOError};
use crate::error::IsIOError::{OnError, StdIoError};
use crate::utils::calculate_size;
use sevenz_rust2::SevenZMethodConfiguration;
use std::path::Path;

pub struct SevenzComde;

impl comde::Compress for SevenzComde {
    type Configs = SevenzConfigs;

    fn compress_file<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        config: Self::Configs,
    ) -> crate::Result<CompressStats> {
        let (input, output) = (input.as_ref(), output.as_ref());
        if !input.exists() {
            return Err(IOError(OnError("Path not exist.".to_string())));
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
        }
        .map_err(|e| CompressError(e.to_string()))?;

        // Write and stop timing.
        writer.finish().map_err(|e| CompressError(e.to_string()))?;
        let compressed_size = std::fs::metadata(output)
            .map_err(|e| IOError(StdIoError(e)))?
            .len();
        let duration = start_time.elapsed();

        Ok(CompressStats::new(origin_size, compressed_size, duration))
    }
}

/// The compression configuration of 7z.
pub struct SevenzConfigs {
    solid_compress: bool,
    methods: Vec<SevenZMethodConfiguration>,
}

impl comde::ComdeConfig for SevenzConfigs {
    fn new() -> SevenzConfigs {
        SevenzConfigs {
            solid_compress: false,
            methods: Vec::new(),
        }
    }
}

impl SevenzConfigs {
    /// Encrypt with AES256.
    pub fn set_password(mut self, password: &str) -> Self {
        self.methods
            .push(sevenz_rust2::AesEncoderOptions::new(password.into()).into());
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

    /// use ZStandard with default arguments.
    pub fn use_zstd(mut self) -> Self {
        let m = sevenz_rust2::ZStandardOptions::default();
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

    /// Use Deflate with given compression level.
    #[cfg(feature = "sevenz-deflate")]
    pub fn use_deflate_with_level(mut self, level: u32) -> Self {
        let m = sevenz_rust2::DeflateOptions::from_level(level);
        self.methods.push(m.into());
        self
    }
}

/// Decompression configuration for 7z.
pub struct SevenzDeConfig {
    password: Option<String>,
}

impl comde::ComdeConfig for SevenzDeConfig {
    fn new() -> SevenzDeConfig {
        SevenzDeConfig { password: None }
    }
}

impl SevenzDeConfig {
    /// Decompress with password.
    pub fn with_password(mut self, password: &str) -> SevenzDeConfig {
        self.password = Some(password.into());
        self
    }
}

impl comde::Decompress for SevenzComde {
    type Configs = SevenzDeConfig;

    fn decompress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> crate::Result<DecompressStats> {
        // start timing.
        let start_time = std::time::Instant::now();
        let compressed_size = std::fs::metadata(&input)
            .map_err(|e| IOError(StdIoError(e)))?
            .len();

        let result = match config.password.as_ref() {
            Some(password) => sevenz_rust2::decompress_file_with_password(
                &input,
                &output,
                password.as_str().into(),
            ),
            None => sevenz_rust2::decompress_file(&input, &output),
        };

        // Handle specific 7z errors
        if let Err(e) = result {
            let error_msg = e.to_string();
            if error_msg.contains("password") || error_msg.contains("Password") {
                if config.password.is_some() {
                    return Err(DecompressError(DecompError::PasswdIncorrect(error_msg)));
                } else {
                    return Err(DecompressError(DecompError::PasswdNeeded(error_msg)));
                }
            } else {
                return Err(DecompressError(DecompError::DecompressErr(e.to_string())));
            }
        }

        // finish and stop timing.
        let duration = start_time.elapsed();
        let decompressed_size = calculate_size(&output)?;
        Ok(DecompressStats::new(
            compressed_size,
            decompressed_size,
            duration,
        ))
    }
}

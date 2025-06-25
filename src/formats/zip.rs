//! `.zip` format support.

use crate::comde;
use crate::comde::{CompressStats, DecompressStats};
use crate::error::DecompError::{PasswdIncorrect, PasswdNeeded};
use crate::error::Error::{CompressError, DecompressError, IOError};
use crate::error::IsIOError::{OnError, StdIoError};
use crate::utils::calculate_size;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{FileOptions, SimpleFileOptions};
use zip::{CompressionMethod, ZipWriter};

pub struct ZipComde;

impl comde::Compress for ZipComde {
    type Configs = ZipConfig;

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
        let origin_size = calculate_size(input)?;

        // Create a file, if the file already exists, truncate it.
        let file = File::create(output).map_err(|e| IOError(StdIoError(e)))?;
        let options = config.file_options;
        let mut zip = ZipWriter::new(file);

        if input.is_file() {
            // Handle single file
            let mut file = File::open(input).map_err(|e| IOError(StdIoError(e)))?;
            let file_name = input.file_name().unwrap().to_string_lossy().to_string();

            zip.start_file(file_name, options)
                .map_err(|e| CompressError(e.to_string()))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| IOError(StdIoError(e)))?;
            zip.write_all(&buffer)
                .map_err(|e| CompressError(e.to_string()))?;
        } else if input.is_dir() {
            // Handle directory
            use walkdir::WalkDir;

            let walkdir = WalkDir::new(input);
            let iter = walkdir.into_iter().filter_map(|e| e.ok());

            for entry in iter {
                let path = entry.path();
                let name = path
                    .strip_prefix(Path::new(input))
                    .unwrap_or(path)
                    .to_string_lossy();

                if path.is_file() {
                    zip.start_file(name.to_string(), options)
                        .map_err(|e| CompressError(e.to_string()))?;
                    let mut file = File::open(path).map_err(|e| IOError(StdIoError(e)))?;
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)
                        .map_err(|e| IOError(StdIoError(e)))?;
                    zip.write_all(&buffer)
                        .map_err(|e| CompressError(e.to_string()))?;
                } else if path != input {
                    // Create directory entries

                    zip.add_directory(name.to_string(), options)
                        .map_err(|e| CompressError(e.to_string()))?;
                }
            }
        }

        // Finish and stop timing
        zip.finish().map_err(|e| CompressError(e.to_string()))?;
        let compressed_size = std::fs::metadata(output)
            .map_err(|e| IOError(StdIoError(e)))?
            .len();
        let duration = start_time.elapsed();

        Ok(CompressStats::new(origin_size, compressed_size, duration))
    }
}

/// The compression configuration of zip.
pub struct ZipConfig {
    level_supported: bool,
    file_options: SimpleFileOptions,
}

impl comde::ComdeConfig for ZipConfig {
    fn new() -> Self {
        ZipConfig {
            level_supported: false,
            file_options: FileOptions::default(),
        }
    }
}

impl ZipConfig {
    /// Choose the compression method.
    pub fn set_method(mut self, method: &str) -> Self {
        use CompressionMethod as cm;
        let m = match method {
            "store" => cm::Stored,

            #[cfg(feature = "zip-zstd")]
            "zstd" => {
                self.level_supported = true;
                cm::Zstd
            }

            #[cfg(feature = "zip-lzma")]
            "lzma" => cm::Lzma,

            #[cfg(feature = "zip-deflate")]
            "deflate" => cm::Deflated,
            _ => cm::Stored,
        };
        let _ = self.file_options.compression_method(m);

        self
    }

    /// Set the compression level.
    ///
    /// Range of values depends on compression method:
    /// * `Deflated`: `10 - 264` for Zopfli, `0 - 9` for other encoders. Default is `24` if Zopfli is the
    ///   only encoder, or `6` otherwise.
    /// * `Bzip2`: `0 - 9`. Default is `6`.
    /// * `Zstd`: `-7 - 22`, with zero being mapped to default level. Default is `3`.
    ///
    /// # Panic
    /// the function will panic when you call it with unsupported compression method.
    pub fn set_level(self, level: i64) -> Self {
        debug_assert!(self.level_supported);
        let _ = self.file_options.compression_level(Some(level));
        self
    }

    /// Set the password with AES256.
    pub fn set_password(self, password: &str) -> Self {
        self.file_options
            .with_aes_encryption(zip::AesMode::Aes256, password);
        self
    }

    /// Enable the large file comde (`>= 4GB`).
    pub fn enable_large_file(self, enable: bool) -> Self {
        let _ = self.file_options.large_file(enable);
        self
    }
}

pub struct ZipDeConfig {
    password: Option<String>,
}

impl comde::ComdeConfig for ZipDeConfig {
    fn new() -> Self {
        ZipDeConfig { password: None }
    }
}

impl ZipDeConfig {
    /// Set the password for encrypted zip files.
    pub fn set_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }
}

impl comde::Decompress for ZipComde {
    type Configs = ZipDeConfig;

    fn decompress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> crate::Result<DecompressStats> {
        use std::io::copy;
        use zip::ZipArchive;

        let (input, output) = (input.as_ref(), output.as_ref());

        // start timing.
        let start_time = std::time::Instant::now();
        let compressed_size = std::fs::metadata(input)
            .map_err(|e| IOError(StdIoError(e)))?
            .len();

        // Open the zip file
        let file = File::open(input).map_err(|e| IOError(StdIoError(e)))?;
        let mut archive = ZipArchive::new(file).map_err(|e| CompressError(e.to_string()))?;

        // Create output directory if it doesn't exist
        if !output.exists() {
            std::fs::create_dir_all(output).map_err(|e| IOError(StdIoError(e)))?;
        }

        // Extract all files
        for i in 0..archive.len() {
            // First check if the file is encrypted
            let is_encrypted = {
                let file_info = archive.by_index_raw(i).map_err(|e| {
                    DecompressError(crate::error::DecompError::DecompressErr(e.to_string()))
                })?;
                file_info.encrypted()
            };

            let mut file = if is_encrypted {
                if let Some(ref password) = config.password {
                    // Try to extract with password
                    match archive.by_index_decrypt(i, password.as_bytes()) {
                        Ok(file) => file,
                        Err(e) => {
                            // If the error is due to an invalid password, return PasswdIncorrect
                            if e.to_string().contains("Invalid password") {
                                return Err(DecompressError(PasswdIncorrect(
                                    "Invalid password".to_string(),
                                )));
                            } else {
                                return Err(DecompressError(
                                    crate::error::DecompError::DecompressErr(e.to_string()),
                                ));
                            }
                        }
                    }
                } else {
                    // File is encrypted but no password provided
                    return Err(DecompressError(PasswdNeeded(
                        "Password required for encrypted file".to_string(),
                    )));
                }
            } else {
                // Extract without password
                archive.by_index(i).map_err(|e| {
                    DecompressError(crate::error::DecompError::DecompressErr(e.to_string()))
                })?
            };

            let outpath = match file.enclosed_name() {
                Some(path) => output.join(path),
                None => continue, // Skip files with invalid names
            };

            if file.name().ends_with('/') {
                // This is a directory
                std::fs::create_dir_all(&outpath).map_err(|e| IOError(StdIoError(e)))?;
            } else {
                // This is a file
                if let Some(parent) = outpath.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).map_err(|e| IOError(StdIoError(e)))?;
                    }
                }

                let mut outfile = File::create(&outpath).map_err(|e| IOError(StdIoError(e)))?;
                copy(&mut file, &mut outfile).map_err(|e| IOError(StdIoError(e)))?;
            }

            // Set file permissions on Unix systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                        .map_err(|e| IOError(StdIoError(e)))?;
                }
            }
        }

        // finish.
        let duration = start_time.elapsed();
        let decompressed_size = calculate_size(output)?;
        Ok(DecompressStats::new(
            compressed_size,
            decompressed_size,
            duration,
        ))
    }
}

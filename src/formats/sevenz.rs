//! `.7z` format support.

use crate::comde;
use crate::comde::CompressStats;
use crate::error::Error::{CompressError, IOError};
use crate::error::IsIOError::{OnError, StdIoError};
use crate::utils::calculate_size;

pub struct SevenzCompressor;

impl comde::Compress for SevenzCompressor {
    type Configs = SevenzConfigs;

    fn compress_file(
        &self,
        input: std::path::PathBuf,
        output: std::path::PathBuf,
        config: Self::Configs
    ) -> crate::Result<CompressStats> {
        if !input.exists() {
            return Err(IOError(OnError("Path not exist.".to_string() )))
        }
        
        let start_time = std::time::Instant::now();
        let origin_size = calculate_size(&input)?;
        
        match config.password {
            None => {
                sevenz_rust2::compress_to_path(&input, &output).map_err(|e|
                CompressError(e.to_string() ))?
            }
            Some(passwd) => {
                sevenz_rust2::compress_to_path_encrypted(&input, &output, passwd.as_str().into()).map_err(|e| {
                    CompressError(e.to_string())
                })?
            }
        }
        
        let compressed_size = std::fs::metadata(output).map_err(|e|
            IOError(StdIoError(e))
        )?.len();
        let duration = start_time.elapsed();
        
        Ok(CompressStats::new(origin_size, compressed_size, duration))
    }
}

pub struct SevenzConfigs {
    password: Option<String>,
}

impl comde::ComdeConfig for SevenzConfigs {
    fn new() -> SevenzConfigs {
        SevenzConfigs {
            password: None,
        }
    }
}

impl SevenzConfigs {
    pub fn set_password(mut self, password: &str) -> Self {
        self.password = Some(password.to_string());
        self
    }

}

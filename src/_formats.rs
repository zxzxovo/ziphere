//! Supported _formats and corresponding functions
//! u
//! and configs for `compress` and `decompress`.

use crate::_comde::ComdeConfig;
use crate::_formats::_sevenz::{SevenzComde, SevenzConfigs, SevenzDeConfig};
use crate::_formats::_zip::{ZipComde, ZipConfig, ZipDeConfig};

/// 7z format implementation.
#[cfg(feature = "sevenz")]
pub mod _sevenz;

/// zip format implementation.
#[cfg(feature = "zip")]
pub mod _zip;

/// The Comde _formats builder.
#[derive(Debug)]
pub enum Formats {
    #[cfg(feature = "zip")]
    Zip(&'static ZipComde),

    #[cfg(feature = "sevenz")]
    SevenZ(&'static SevenzComde),
}
impl Formats {
    /// Get a new compressor.
    pub fn new_with(format: &str) -> Formats {
        match format {
            "zip" | "ZIP" | "Zip" => Formats::Zip(&ZipComde),
            "7z" | "7Z" | "sevenz" | "sevenZ" | "SevenZ" => Formats::SevenZ(&SevenzComde),
            _ => panic!("Unsupported format."),
        }
    }
}

/// The Configuration builder.
#[derive(Debug)]
pub enum Configs {
    #[cfg(feature = "zip")]
    ZipConfig(ZipConfig),
    #[cfg(feature = "zip")]
    ZipDeConfig(ZipDeConfig),

    #[cfg(feature = "sevenz")]
    SevenzConfig(SevenzConfigs),
    #[cfg(feature = "sevenz")]
    SevenzDeConfig(SevenzDeConfig),
}

impl Configs {
    pub fn new_compress(format: &str) -> Configs {
        match format {
            "zip" | "ZIP" | "Zip" => Configs::ZipConfig(ZipConfig::new()),
            "7z" | "7Z" | "sevenz" | "sevenZ" | "SevenZ" => {
                Configs::SevenzConfig(SevenzConfigs::new())
            }
            _ => panic!("Unsupported format."),
        }
    }

    pub fn new_decompress(format: &str) -> Configs {
        match format {
            "zip" | "ZIP" | "Zip" => Configs::ZipDeConfig(ZipDeConfig::new()),
            "7z" | "7Z" | "sevenz" | "sevenZ" | "SevenZ" => {
                Configs::SevenzDeConfig(SevenzDeConfig::new())
            }
            _ => panic!("Unsupported format."),
        }
    }
}

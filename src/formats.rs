//! TODO

#[cfg(feature = "sevenz")]
pub mod sevenz;

#[cfg(feature = "zip")]
pub mod zip;

pub struct Formats;

impl Formats {

    #[cfg(feature = "zip")]
    pub fn get_zip() -> zip::ZipComde {
        zip::ZipComde
    }

    #[cfg(feature = "sevenz")]
    pub fn get_7z() -> sevenz::SevenZComde {
        sevenz::SevenZComde
    }
}

pub struct Configs;

impl Configs {
    #[cfg(feature = "zip")]
    pub fn get_zip() -> zip::ZipCfg {
        zip::ZipCfg::new()
    }

    #[cfg(feature = "sevenz")]
    pub fn get_7z() -> sevenz::SevenZCfg {
        sevenz::SevenZCfg::new()
    }
}
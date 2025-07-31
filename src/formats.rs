//! TODO

pub mod sevenz;
pub mod zip;


pub enum Formats {
    SevenZ,
    Zip
}

impl Formats {
    pub fn get_support_formats() -> &'static[&'static str] {
        &["SevenZ", "Zip"][..]
    }
}
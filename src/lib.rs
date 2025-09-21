//! # ZipHere
//!
//! TODO

pub mod formats;
mod error;

pub(crate) mod utils;
pub mod comde;
pub mod view;

pub fn run_test() {
    let a = formats::Formats::get_zip();
    let b = formats::Formats::get_7z();
    
    let ac = formats::Configs::get_zip();
    let bc = formats::Configs::get_7z().set_solid_compress(true);
    
    b.compress("a", "b", &bc).unwrap();
    
    
}
//! # ZipHere
//!
//! TODO

pub mod formats;
mod error;

pub(crate) mod utils;
pub mod comde;
pub mod view;

pub fn run_test() {
    let _a = formats::Formats::get_zip();
    let b = formats::Formats::get_7z();
    
    let _ac = formats::Configs::get_zip();
    let bc = formats::Configs::get_7z().set_solid_compress(true);
    
    b.compress_f(&["a"], "b", &bc).unwrap();
    
    
}
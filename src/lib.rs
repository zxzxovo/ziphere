//! # ZipHere
//!
//! TODO

pub mod formats;
pub mod error;
pub mod utils;
pub mod comde;
pub mod view;

pub use formats::Formats;
pub use formats::Configs;

pub fn run_test() {

    let zc = Configs::get_7z().set_password("123456");
    let _zc_np = Configs::get_7z();
    let z = Formats::get_7z().decompress_f("F:\\RustProjects\\ziphere\\Desktop.7z", "OUR", &zc).unwrap();
    println!("Status: {}", z);
    
    
}
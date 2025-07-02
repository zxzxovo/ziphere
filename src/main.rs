use ziphere::{comde::{Compress, Decompress}, Configs, Formats};


fn main() {
    println!("Hello, world!");

    if let Formats::Zip(zip) = Formats::new_with("zip")
        && let Configs::ZipConfig(zipc) = Configs::new_compress("zip") 
    {
        let result = zip.compress_file("input", "output", zipc).unwrap();
        println!("{}", result);
    }
}

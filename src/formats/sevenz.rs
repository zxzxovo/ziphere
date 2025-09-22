//! # 7Z Support.
//! We have 2 struct here:
//! - SevenZComde (ZST)
//! - SevenZCfg
//! 

use crate::comde::{CompressStatus, DecompressStatus};
use crate::error::ComdeError;
use crate::utils;
use std::io::{Read, Write};
use std::io::Seek;
use std::path::Path;
use std::time::Instant;

pub struct SevenZComde;

pub fn todo() {
    todo!("View to be implemented")
}

impl SevenZComde {

    /// Compress files to an archive and then write into a writer.
    pub fn compress<P: AsRef<Path>, W: Write + Seek>(
        self,
        input: &[P],
        output: W,
        config: &SevenZCfg 
    ) -> Result<CompressStatus, ComdeError> {
        let cw = utils::CountingWriter::new(output);
        let mut writer = sevenz_rust2::ArchiveWriter::new(cw.share()).map_err(|e| ComdeError::SevenZError(e.to_string()))?;

        let time_begin = Instant::now();
        let size_origin = utils::size_of_inarray(&input).map_err(|e| ComdeError::SevenZError(e.to_string()))?;

        writer.set_content_methods(config.configs.clone());
        if config.solid_compress {
            for p in input {
                writer.push_source_path(p, |_| true).map_err(|e| ComdeError::SevenZError(e.to_string()))?;
            }
        } else {
            for p in input {
                writer.push_source_path_non_solid(p, |_| true).map_err(|e| ComdeError::SevenZError(e.to_string()))?;
            }
        }
        writer.finish().map_err(|e| ComdeError::SevenZError(format!("e: {}\n {}", e.to_string(), "Writing files error.")))?;
        
        let size_compressed = cw.bytes_written();
        let time_finished = time_begin.elapsed();

        Ok(CompressStatus::new(size_origin, size_compressed, time_finished))
    }

    /// Compress files to an archive and then write into a file of given path.
    pub fn compress_f<P: AsRef<Path>>(
        self,
        input: &[P],
        output: P,
        config: &SevenZCfg,
    ) -> Result<CompressStatus, ComdeError> {
        let output  = output.as_ref();
        let out_file = std::fs::File::create(output).map_err(|e| ComdeError::SevenZError(e.to_string()))?;
        self.compress(input, out_file, config)
    }

    /// Decompress an archive from a reader and write into a file.
    pub fn decompress<P: AsRef<Path>, R: Read + Seek>(
        self,
        input: R,
        output: P,
        config: &SevenZCfg
    ) -> Result<DecompressStatus, ComdeError> {
        let time_begin = Instant::now();
        sevenz_rust2::decompress_with_password(input, output, config.password.clone()).map_err(|e| ComdeError::SevenZError(e.to_string()))?;
        let time_finished = time_begin.elapsed();
        Ok(DecompressStatus::new(1, 1, time_finished))
    }

    /// Decompress an archive of given path and write into a file.
    pub fn decompress_f<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &SevenZCfg,
    ) -> Result<DecompressStatus, ComdeError> {
        let input = input.as_ref();
        let in_file = std::fs::File::open(input).map_err(|e| ComdeError::SevenZError(e.to_string()))?;
        self.decompress(in_file, output, config)
    }
}

pub struct SevenZCfg {
    solid_compress: bool,
    password: sevenz_rust2::Password,
    configs: Vec<sevenz_rust2::EncoderConfiguration>
}

impl SevenZCfg {

    /// # Create a SevenZCfg.
    /// When you are not setting other algorithm params,
    /// this crate uses `lzma2` as default.
    /// You can use `use_lzma2_with_level()` to set its level.
    /// 
    /// # Method Chaning
    /// When you try to call mutliple algorithm functions,
    /// we only save your last call and its settings.
    pub fn new() -> SevenZCfg {
        SevenZCfg {
            solid_compress: false,
            password: sevenz_rust2::Password::empty(),
            configs: vec![]
        }
    }

    /// Switch wheter to enable solid compress, which may make compressed file smaller,
    /// but also lead to longer compress time.
    pub fn set_solid_compress(mut self, enabled: bool) -> SevenZCfg {
        self.solid_compress = enabled;
        self
    }

    /// Set the password for compressing or decompressing, encrypt goes with AES256.
    pub fn set_password(mut self, password: &str) -> SevenZCfg {
        self.password = sevenz_rust2::Password::new(password);
        self
    }

    /// Use LZMA2 to compress.
    /// Level range to 0 - 9 .
    pub fn use_lzma2_with_level(mut self, level: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::Lzma2Options::from_level(level);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use LZMA2 to compress.
    /// Level range to 0 - 9 .
    /// # Arguments
    /// * `level` - Compression level (0-9, clamped to this range)
    /// * `threads` - Count of threads used to compress the data
    /// * `chunk_size` - Size of each independent chunk of uncompressed data.
    ///   The more streams can be created, the more effective is
    ///   the multi threading, but the worse the compression ratio
    ///   will be (value will be clamped to have at least the size of the dictionary).
    pub fn use_lzma2_with_multithread(mut self, level: u32, threads: u32, chunk_size: u64) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::Lzma2Options::from_level_mt(level, threads, chunk_size);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use Delta to compress. This is also the default compress algorithm.
    /// Distance range to 1 - 256 .
    pub fn use_delta_with_distance(mut self, distance: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::DeltaOptions::from_distance(distance);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use Zstd to compress.
    /// Level range: 0 - 22 .
    #[cfg(feature="sevenz-zstd")]
    pub fn use_zstd_with_level(mut self, level: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::ZstandardOptions::from_level(level);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use Deflate to compress.
    /// Level range to 0 - 9 .
    #[cfg(feature="sevenz-deflate")]
    pub fn use_deflate_with_level(mut self, level: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::DeflateOptions::from_level(level);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }
    
    /// Use Bzip2 to compress.
    /// Level range to 1 - 9 .
    #[cfg(feature = "sevenz-bzip2")]
    pub fn use_bzip2_with_level(mut self, level: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::Bzip2Options::from_level(level);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use PPMD to compress.
    /// Level range to 0 - 9 .
    #[cfg(feature = "sevenz-ppmd")]
    pub fn use_ppmd_with_level(mut self, level: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::PpmdOptions::from_level(level);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use PPMD to compress.
    /// Set model order and memory size Mannuly using this function.
    #[cfg(feature = "sevenz-ppmd")]
    pub fn use_ppmd_with_order_memsize(mut self, order: u32, memory_size: u32) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::PpmdOptions::from_order_memory_size(order, memory_size);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use PPMD to compress.
    /// This function uses:
    /// order: 3
    /// size: 1 MB
    #[cfg(feature = "sevenz-ppmd")]
    pub fn use_ppmd_fast(mut self) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::PpmdOptions::from_order_memory_size(3, 1 << 20);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use PPMD to compress.
    /// This function uses:
    /// order: 8
    /// size: 8 MB
    #[cfg(feature = "sevenz-ppmd")]
    pub fn use_ppmd_balanced(mut self) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::PpmdOptions::from_order_memory_size(8, 1 << 23);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }

    /// Use PPMD to compress.
    /// This function uses:
    /// order: 16
    /// size: 32 MB
    #[cfg(feature = "sevenz-ppmd")]
    pub fn use_ppmd_max(mut self) -> SevenZCfg {
        let cfg = sevenz_rust2::encoder_options::PpmdOptions::from_order_memory_size(16, 1 << 25);
        self.configs.clear();
        self.configs.push(cfg.into());
        self
    }


}

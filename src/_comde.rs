//! Interface for compressing, decompressing and archive operations.

use crate::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// Common configs for compression or decompression.
pub trait ComdeConfig: Sized {
    /// New a configuration struct.
    fn new() -> Self;

    /// Finally do something and then return
    fn build(self) -> Self {
        self
    }
}

/// The _formats that support `compress` option should implement.
pub trait Compress {
    type Configs: ComdeConfig;
    fn compress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> Result<CompressStats>;
}

/// The _formats that support `decompress` option should implement.
pub trait Decompress {
    type Configs: ComdeConfig;
    fn decompress_file<P: AsRef<Path>>(
        input: P,
        output: P,
        config: Self::Configs,
    ) -> Result<DecompressStats>;
}

/// The stats that this compression procession have.
pub struct CompressStats {
    pub origin_size: u64,
    pub compressed_size: u64,
    pub time_cost: Duration,
}

impl std::fmt::Display for CompressStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Origin size: {}\nCompressed size: {}\nTime cost: {}s\n",
            self.origin_size,
            self.compressed_size,
            self.time_cost.as_secs()
        )
    }
}

impl CompressStats {
    pub(crate) fn new(origin_size: u64, compressed_size: u64, time_cost: Duration) -> Self {
        Self {
            origin_size,
            compressed_size,
            time_cost,
        }
    }
}
/// The stats that this decompression procession have.
pub struct DecompressStats {
    pub compressed_size: u64,
    pub decompressed_size: u64,
    pub time_cost: Duration,
}

impl DecompressStats {
    pub(crate) fn new(compressed_size: u64, decompressed_size: u64, time_cost: Duration) -> Self {
        Self {
            compressed_size,
            decompressed_size,
            time_cost,
        }
    }
}

impl std::fmt::Display for DecompressStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Origin size: {}\nDecompressed size: {}\nTime cost: {}s\n",
            self.compressed_size,
            self.decompressed_size,
            self.time_cost.as_secs()
        )
    }
}

/// The format that needs to support non-decompress view should implement this.
pub trait ArchiveOpener {
    /// Create an Archive<'a> .
    fn open_view<P>(path: P) -> Result<Archive>
    where
        P: AsRef<Path>;
}

/// The archive that is to manage the viewer procession.
pub struct Archive {
    /// The path of the archive.
    path: PathBuf,

    /// Relative path of the archive.
    i_path: PathBuf,

    /// The viewer that will be used to view the archive.
    /// It should be a static reference to a viewer implementation.
    viewer: &'static dyn Viewer,

    /// The size of the archive in bytes.
    size: u64,
}

impl Archive {
    pub(crate) fn create(path: PathBuf, viewer: &'static dyn Viewer) -> Result<Self> {
        let size = crate::_utils::calculate_size(&path)?;
        let i_path = PathBuf::from("/");
        Ok(Self { path, i_path, viewer, size })
    }

    // Change the current path of the archive.
    // The path is relative to the archive root.
    // If the path is empty or ".", it stays at the current directory.
    // If the path is "..", it moves up one directory.
    // If the path is a valid directory, it changes to that directory.
    // If the path does not exist or is not a directory, it returns an error.
    // pub fn cd<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    //     let path = path.as_ref();
    //     match path.as_ref() {
    //         Path::new("") | Path::new(".") => {
    //             // No change, stay at the current directory.
    //             Ok(())
    //         },
    //         Path::new("..") => {
    //             // Move up one directory.
    //             if self.i_path == PathBuf::from("/") {
    //                 return Err(crate::_error::Error::ViewError(
    //                     crate::_error::ViewError::Unsupported,
    //                 ));
    //             }
    //             self.i_path = self.i_path.parent().unwrap_or(&PathBuf::from("/")).to_path_buf();
    //             Ok(())
    //         },
    //         p => {
    //             // Change to the specified directory.
    //             let new_path = self.i_path.join(p);
    //             if self.viewer.path_exists(&new_path).is_none() || !new_path.is_dir() {
    //                 return Err(crate::_error::Error::ViewError(
    //                     crate::_error::ViewError::NotFound,
    //                 ));
    //             }
    //             self.i_path = new_path;
    //             Ok(())
    //         },
    //     }
    // }
}

/// The trait defines the actual operations to view the archive.
/// The path used inside a viewer is **relative** to the archive root.
pub trait Viewer {
    // Functions to be implemented.


    fn get_entries(&self, path: &PathBuf) -> Result<Vec<ViewEntry>>;

    fn add_entry_to(&mut self, path_from: &PathBuf, to: &PathBuf) -> Result<CompressStats>;

    fn remove_entry_at(&mut self, path_at: &PathBuf) -> Result<()>;

    fn decompress_file(&self, path_at: &PathBuf, to: &PathBuf) -> Result<DecompressStats>;

    // Functions support.
    // fn add_entry_to_root(&mut self, path_from: &PathBuf) -> Result<CompressStats> {
    //     self.add_entry_to(path_from, self.path().to_path_buf())
    // }

    fn path_exists(&self, path: &PathBuf) -> Option<Vec<ViewEntry>> {
        match self.get_entries(path) {
            Ok(entries) => Some(entries),
            Err(crate::_error::Error::ViewError(crate::_error::ViewError::NotFound)) => None,
            Err(_) => None,
        }
    }
}

/// Archive entry information.
#[derive(Debug, Clone)]
pub struct ViewEntry {
    /// Path of the entry within the archive.
    pub path: String,
    /// Whether this entry is a directory.
    pub is_dir: bool,
    /// Compressed size in bytes.
    pub compressed_size: u64,
    /// Uncompressed size in bytes.
    pub uncompressed_size: u64,
    /// Last modified time.
    pub modified_time: Option<SystemTime>,
    /// Whether the entry is encrypted.
    pub is_encrypted: bool,
    /// CRC32 checksum (if available).
    pub crc32: Option<u32>,
}

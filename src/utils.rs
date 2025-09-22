//! Utils
//!

use std::{cell::RefCell, io::{Seek, Write, Read}, path::Path, rc::Rc};
use crate::error::AppError;
use walkdir::WalkDir;

/// The wrapper for type [std::io::Write] .
/// We use this to count written bytes.
pub struct CountingWriter<W: Write> {
    inner: Rc<RefCell<W>>,
    bytes_written: Rc<RefCell<u64>>,
}

impl <W: Write> CountingWriter<W> {
    /// Create a new CountingWriter.
    pub fn new(writer: W) -> Self {
        Self { 
            inner: Rc::new(RefCell::new(writer)),
            bytes_written: Rc::new(RefCell::new(0)) }
    }

    /// Get the written bytes of the Writer.
    pub fn bytes_written(&self) -> u64 {
        *self.bytes_written.borrow()
    }

    /// Return a new CountingWriter that shares inner data.
    pub fn share(&self) -> Self {
        Self { 
            inner: Rc::clone(&self.inner), 
            bytes_written: Rc::clone(&self.bytes_written)
        }
    } 

    /// Get the inside Write Object.
    /// This consumes the CountingWriter and may fail if other reference exists.
    /// If failed, it will return itself wrapped in Err() .
    #[warn(dead_code)]
    pub fn try_into_inner(self) -> Result<W, Self> {
        match Rc::try_unwrap(self.inner) {
            Ok(cell) => Ok(cell.into_inner()),
            Err(rc) => Err(Self { inner: rc, bytes_written: self.bytes_written })
        }
    }
}

impl <W: Write> Write for CountingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_written = self.inner.borrow_mut().write(buf)?;
        *self.bytes_written.borrow_mut() += bytes_written as u64;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.borrow_mut().flush()
    }
}

impl <W: Write + Seek> Seek for CountingWriter<W> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        // Seek changes only position, did **not** write bytes.
        self.inner.borrow_mut().seek(pos)
    }
}

/// The wrapper for type [std::io::Read] .
/// We use this to count written bytes.
pub struct CountingReader<R: Read> {
    inner: Rc<RefCell<R>>,
    bytes_read: Rc<RefCell<u64>>,
}

impl <R: Read> CountingReader<R> {
    /// Create a new CounringReader.
    pub fn new(reader: R) -> Self {
        Self { 
            inner: Rc::new(RefCell::new(reader)),
            bytes_read: Rc::new(RefCell::new(0)),
        }
    }

    /// Get the read bytes of the Reader.
    pub fn bytes_read(&self) -> u64 {
        *self.bytes_read.borrow()
    }

    /// Return a new CountingReader that shares inner data.
    pub fn share(&self) -> Self {
        Self { inner: Rc::clone(&self.inner), bytes_read: Rc::clone(&self.bytes_read) }
    }

    /// Get the inside Read Object.
    /// This consumes the CountingReader and may fail if other reference exists.
    /// If failed, it will return itself wrapped in Err() .
    pub fn try_into_inner(self) -> Result<R, Self> {
        match Rc::try_unwrap(self.inner) {
            Ok(cell) => Ok(cell.into_inner()),
            Err(rc) => Err(Self { inner: rc, bytes_read: self.bytes_read })
        }
    }
}

impl <R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.inner.borrow_mut().read(buf)?;
        *self.bytes_read.borrow_mut() += bytes_read as u64;
        Ok(bytes_read)
    }
}

impl <R: Read + Seek> Seek for CountingReader<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        // Seek changes only position, did **not** read bytes.
        self.inner.borrow_mut().seek(pos)
    }
}


/// Calculate the size of the given path, 
/// whether it's a file or direcrory.
pub fn size_of<P: AsRef<Path>>(path: P) -> Result<u64, AppError> {
    let path = path.as_ref();
    if path.is_file() {
        Ok(std::fs::metadata(path)
            .map_err(|e| AppError::FsError(e.to_string().to_string()))?
            .len())
    } else if path.is_dir() {
        let mut total_size = 0u64;
        for entry in WalkDir::new(path) {
            let entry = entry.map_err(|e| AppError::FsError(e.to_string()))?;
            if entry.file_type().is_file() {
                total_size += entry.metadata().map_err(|e| AppError::FsError(e.to_string()))?.len();
            }
        }
        Ok(total_size)
    } else {
        Err(AppError::FsError("The input path is not a file.".to_string()))
    }
}

/// Calculate the size of all paths that given in an array,
/// whether there's files or directorys.
pub fn size_of_inarray<P: AsRef<Path>>(paths: &[P]) -> Result<u64, AppError> {
    paths.iter().try_fold(0, |acc, x| {
        Ok(acc + size_of(x)?)
    })
}

/// Convert bytes to human readable format。
pub fn h_size(size_bytes: u64) -> String {
    match size_bytes {
        0..=1023 => format!("{} B", size_bytes),
        1024..=1048575 => format!("{:.2} KB", size_bytes as f64 / 1024.0),
        1048576..=1073741823 => format!("{:.2} MB", size_bytes as f64 / 1048576.0),
        1073741824..=1099511627775 => format!("{:.2} GB", size_bytes as f64 / 1073741824.0),
        1099511627776..=1125899906842623 => format!("{:.2} TB", size_bytes as f64 / 1099511627776.0),
        _ => format!("{:.2} PB", size_bytes as f64 / 1125899906842624.0),
    }
}
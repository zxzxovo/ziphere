//! Utils
//!

use std::{cell::RefCell, io::{Seek, Write}, path::Path, rc::Rc};
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

pub fn size_of_inarray<P: AsRef<Path>>(paths: &[P]) -> Result<u64, AppError> {
    paths.iter().try_fold(0, |acc, x| {
        Ok(acc + size_of(x)?)
    })
}
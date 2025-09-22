//! # zip support
//! 
//! [todo]

use crate::comde::{CompressStatus, DecompressStatus};
use crate::error::ComdeError::Other;
use crate::error::{ComdeError, Unimplemented};
use std::io::{Seek, Write, Read};
use std::path::Path;

pub struct ZipComde;

impl ZipComde {

    /// Compress files to an archive and then write into a writer.
    pub fn compress<P: AsRef<Path>, W: Write + Seek>(
        self,
        input: &[P],
        output: W,
        config: &ZipCfg 
    ) -> Result<CompressStatus, ComdeError> {
        todo!()
    }

    pub fn compress_f<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &ZipCfg,
    ) -> Result<CompressStatus, ComdeError> {
        let (input, output) = (input.as_ref(), output.as_ref());
        Err(Other(Unimplemented::Unfinished))
    }

    /// Decompress an archive from a reader and write into a file.
    pub fn decompress<P: AsRef<Path>, R: Read + Seek>(
        self,
        input: R,
        output: P,
        config: &ZipCfg,
    ) -> Result<DecompressStatus, ComdeError> {
        todo!()
    }

    pub fn decompress_f<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &ZipCfg,
    ) -> Result<DecompressStatus, ComdeError> {
        let (input, output) = (input.as_ref(), output.as_ref());
        Err(Other(Unimplemented::Unfinished))
    }
}

pub struct ZipCfg;

impl ZipCfg {
    
    pub fn new() -> ZipCfg {
        ZipCfg
    }
}

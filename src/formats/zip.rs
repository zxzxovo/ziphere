//! TODO

use crate::comde::{CompressStatus, DecompressStatus};
use crate::error::ComdeError::Other;
use crate::error::{ComdeError, Unimplemented};
use std::path::Path;

pub struct ZipComde;

impl ZipComde {
    pub fn compress<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &ZipCfg,
    ) -> Result<CompressStatus, ComdeError> {
        let (input, output) = (input.as_ref(), output.as_ref());
        Err(Other(Unimplemented::Unfinished))
    }

    pub fn decompress<P: AsRef<Path>>(
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

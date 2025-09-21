//! TODO

use crate::comde::{CompressStatus, DecompressStatus};
use crate::error::ComdeError::Other;
use crate::error::{ComdeError, Unimplemented};
use std::path::Path;

pub struct SevenZComde;

impl SevenZComde {
    pub fn compress<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &SevenZCfg,
    ) -> Result<CompressStatus, ComdeError> {
        let (input, output) = (input.as_ref(), output.as_ref());

        Err(Other(Unimplemented::Unfinished))
    }

    pub fn decompress<P: AsRef<Path>>(
        self,
        input: P,
        output: P,
        config: &SevenZCfg,
    ) -> Result<DecompressStatus, ComdeError> {
        let (input, output) = (input.as_ref(), output.as_ref());

        Err(Other(Unimplemented::Unfinished))
    }
}

pub struct SevenZCfg {
    solid_compress: bool,
    password: Option<String>,
}

impl SevenZCfg {

    pub fn new() -> SevenZCfg {
        SevenZCfg {
            solid_compress: false,
            password: None,
        }
    }

    /// S
    pub fn set_solid_compress(mut self, enabled: bool) -> SevenZCfg {
        self.solid_compress = enabled;
        self
    }

    /// Encrypt with AES256.
    pub fn set_password(mut self, password: String) -> SevenZCfg {
        self.password = Some(password);
        self
    }
}

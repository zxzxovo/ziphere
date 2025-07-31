//! TODO

use std::path::Path;
use crate::comde::{Comde, ComdeCfg, CompressStatus, DecompressStatus, ComdeResult};

pub struct ZipComde;

impl Comde for ZipComde {
    type Config = ZipCfg;

    fn compress(input: impl AsRef<Path>, output: impl AsRef<Path>, config: &Self::Config) -> ComdeResult<CompressStatus> {
        todo!()
    }

    fn decompress(input: impl AsRef<Path>, output: impl AsRef<Path>, config: &Self::Config) -> ComdeResult<DecompressStatus> {
        todo!()
    }
}

pub struct ZipCfg;

impl ComdeCfg for ZipCfg {

}



use crate::view::{Viewer, ViewOpener, Archive, };

impl ViewOpener for ZipCfg {
    fn open_view() -> crate::view::ViewResult<Archive> {
        todo!()
    }
}

impl Viewer for ZipCfg {
    fn name() -> &'static str {
        todo!()
    }
}
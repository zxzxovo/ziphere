//! TODO

use std::path::Path;
use crate::comde::{Comde, ComdeCfg, CompressStatus, DecompressStatus, ComdeResult};

pub struct SevenZ;

impl Comde for SevenZ {
    type Config = SevenZCfg;

    fn compress(input: impl AsRef<Path>, output: impl AsRef<Path>, config: &Self::Config) -> ComdeResult<CompressStatus> {
        todo!()
    }

    fn decompress(input: impl AsRef<Path>, output: impl AsRef<Path>, config: &Self::Config) -> ComdeResult<DecompressStatus> {
        todo!()
    }
}

pub struct SevenZCfg;

impl ComdeCfg for SevenZCfg {

}



use crate::view::{Viewer, ViewOpener, Archive, ViewResult};

impl ViewOpener for SevenZCfg {
    fn open_view() -> ViewResult<Archive> {
        todo!()
    }
}

impl Viewer for SevenZCfg {
    fn name() -> &'static str {
        todo!()
    }
}
use crate::file_types::CrankFile;

pub struct CrankOpts {
    pub from: &'static CrankFile,
    pub to: &'static CrankFile
}
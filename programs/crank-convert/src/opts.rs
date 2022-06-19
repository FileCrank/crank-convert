use crate::file_types::CrankFileType;

pub struct CrankOpts {
    pub from: &'static CrankFileType,
    pub to: &'static CrankFileType,
}

use crate::file_types::CrankFileType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CSV: CrankFileType = CrankFileType {
        name: "CSV",
        extensions: vec!["csv"],
        conversions: HashMap::new()
    };
}

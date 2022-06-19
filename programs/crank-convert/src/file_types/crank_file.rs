use crate::conversions::ConversionChain;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// A crank file type is a type of file supported by the system, and metadata associated with it
#[derive(Debug)]
pub struct CrankFileType {
    pub name: &'static str,
    pub extensions: Vec<&'static str>,
    pub conversions: HashMap<&'static str, ConversionChain>,
}

impl Display for CrankFileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

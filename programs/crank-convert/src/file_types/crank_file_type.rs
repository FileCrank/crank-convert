use crate::conversion::ConversionChain;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

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

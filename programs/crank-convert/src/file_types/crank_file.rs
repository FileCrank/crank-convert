use std::collections::HashMap;
use crate::conversions::ConversionChain;

/// A crankfile is a type of file supported by the system, and metadata associated with it
pub struct CrankFile {
    pub name: &'static str,
    pub extensions: Vec<&'static str>,
    pub conversions: HashMap<&'static str, ConversionChain>
}
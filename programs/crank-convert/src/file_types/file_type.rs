use crate::conversions::{Conversion, ConversionMap};
use crate::formats::data_format::DataFormat;
use crate::formats::data_holder::DataHolder;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use phf::Set;


pub struct FileType {
    // Names should be unique across the project,
    pub name: &'static str,
    pub extensions: Set<&'static str>,
    pub initialize: fn(&mut DataHolder) -> DataFormat,
    pub empty: fn() -> DataFormat,
    pub conversions: [(&'static FileType, Conversion)]
}

impl PartialEq for FileType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

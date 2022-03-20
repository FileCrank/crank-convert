use crate::conversions::Conversion;
use crate::formats::data_format::DataFormat;
use crate::formats::data_holder::DataHolder;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[macro_export]
/// Instantiate conversions with map syntax
macro_rules! converts_to {
     ($($to: ident => $how: ident),* $(,)?) => {
        ::std::collections::HashMap::from([
            $(($to.deref(), $how as crate::conversions::Conversion),)*
        ])
    }
}

/// A HashMap mapping
pub type ConversionMap<'a> = HashMap<&'a FileType<'a>, Conversion>;

pub struct FileType<'a> {
    // Names should be unique across the project,
    pub name: &'static str,
    pub extensions: HashSet<&'static str>,
    pub conversions: ConversionMap<'a>,

    pub initialize: Box<fn(&mut DataHolder) -> DataFormat>,
    pub empty: Box<fn() -> DataFormat>,
}

impl<'a> PartialEq for FileType<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for FileType<'a> {}

impl<'a> Hash for FileType<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use crate::conversions::Conversion;
use crate::formats::data_format::{DataFormat};
use crate::formats::data_holder::DataHolder;

#[macro_export]
/// Instantiate conversions with map syntax
macro_rules! converts_to {
     ($($to: ident => $how: ident),* $(,)?) => {
        &crate::file_types::file_type::ConversionMap::from([
            $(($to.into(), Box::new($how)),)*
        ])
    }
}

/// A HashMap mapping
pub type ConversionMap = HashMap<&'static FileType,
    Box<Conversion<dyn DataFormat, dyn DataFormat>>
>;

#[derive(Eq, PartialEq)]
pub struct FileType {
    // Names should be unique across the project,
    pub name: &'static str,
    pub extensions: HashSet<&'static str>,
    pub conversions: ConversionMap,

    pub initialize: Box<fn(&DataHolder) -> dyn DataFormat>,
    pub empty: Box<fn() -> dyn DataFormat>
}

/*
impl PartialEq<Self> for FileType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T: DataFormat> Eq for FileType<T> {}


 */

impl Hash for FileType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
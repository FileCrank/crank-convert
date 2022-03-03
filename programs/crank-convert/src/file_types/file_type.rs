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
pub type ConversionMap = HashMap<&'static FileType<dyn DataFormat>,
    Box<Conversion<dyn DataFormat, dyn DataFormat>>
>;

pub struct FileType<T: ?Sized>
where T: DataFormat {
    // Names should be unique across the project,
    pub name: &'static str,
    pub extensions: HashSet<&'static str>,
    pub conversions: ConversionMap,

    pub initialize: Box<fn(&DataHolder) -> T>,
    pub empty: Box<fn() -> T>
}

impl<T> Hash for FileType<T>
where T: DataFormat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
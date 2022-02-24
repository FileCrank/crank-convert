use std::collections::HashSet;
use crate::Opts;

/// The master trait that defines how one data type can be converted into another -
/// each file type will implement this, along with the
pub trait Convertable {
    /// The name of the data type. Must be unique across all impl Convertable
    const NAME: &'static str;

    /// Whether a particular conversion is supported for this datatype
    fn is_supported(&self, target: &'static str) -> bool {
        self.supported_conversions().contains(target)
    }

    /// The extensions that are recognized for this data type
    fn extensions(&self) -> HashSet<&'static str>;

    /// The types that this can be converted into
    fn supported_conversions(&self) -> HashSet<&'static str>;
}
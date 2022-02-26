use crate::Opts;
use std::collections::HashSet;
use std::io::{BufReader, Read, Write};
use crate::errors::Result;
use phf;

/// The master trait that defines how one data type can be converted into another -
/// each file type will implement this, along with the
pub trait Convertable {
    /// The name of the data type. Must be unique across all impl Convertable
    const NAME: &'static str;

    fn get_name(&self) -> &'static str {
        return Self::NAME;
    }

    /// Whether a particular conversion is supported for this datatype
    fn is_supported(&self, target: &'static str) -> bool {
        self.supported_conversions().contains(target)
    }

    /// The extensions that are recognized for this data type
    fn extensions(&self) -> phf::Set<&'static str>;

    /// The types that this can be converted into
    fn supported_conversions(&self) -> phf::Set<&'static str>;
}
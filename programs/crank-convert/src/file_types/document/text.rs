use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::{converts_to, set};
use crate::file_types::file_type::FileType;
use crate::conversions::simple_copy;
use crate::file_types::document::html::HTML;
use crate::file_types::file_type::ConversionMap;
use crate::formats::data_format::{empty_bytes, initialize_bytes};

lazy_static! {

    pub static ref conversions: ConversionMap<'static> = HashMap::from([
        (HTML.deref(), Box::new(simple_copy))
    ]);
    pub static ref TXT: FileType<'static> = FileType {
        name: "TXT",
        extensions: set!["txt", "text"],
        conversions,
        initialize: Box::new(initialize_bytes),
        empty: Box::new(empty_bytes)
    };
}
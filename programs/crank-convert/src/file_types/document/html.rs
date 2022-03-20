use crate::conversions::simple_copy;
use crate::file_types::document::text::TXT;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::{empty_bytes, initialize_bytes};
use crate::{converts_to, set};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref HTML: FileType<'static> = FileType {
        name: "HTML",
        extensions: set!["htm", "html"],
        conversions: converts_to! {
            TXT => simple_copy
        },
        initialize: Box::new(initialize_bytes),
        empty: Box::new(empty_bytes)
    };
}

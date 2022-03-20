use lazy_static::lazy_static;
use crate::{converts_to, set};
use crate::file_types::file_type::{FileType};
use crate::conversions::simple_copy;
use crate::file_types::document::text::TXT;
use crate::formats::data_format::{initialize_bytes, empty_bytes};

lazy_static! {
    pub static ref HTML: FileType<'static> = FileType{
        name: "HTML",
        extensions: set!["htm", "html"],
        conversions: converts_to!{
            TXT => simple_copy
        },
        initialize: initialize_bytes,
        empty: empty_bytes
    };
}

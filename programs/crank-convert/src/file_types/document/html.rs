use crate::conversions::simple_copy;
use crate::conversions::conversions::{ConversionQuality, ConversionStep, Conversion};
use crate::file_types::file_type::FileType;
use crate::formats::data_format::{empty_bytes, initialize_bytes};
use crate::{set};
use lazy_static::lazy_static;
use phf::{phf_set, phf_map};
use crate::file_types::document::text::TXT;


pub const HTML: FileType = FileType {
    name: "HTML",
    extensions: phf_set!["htm", "html"],
    initialize: initialize_bytes,
    empty: empty_bytes,
    conversions:  [
        (&TXT, Conversion {
            steps: *[
                ConversionStep {
                    execute: simple_copy,
                    quality: ConversionQuality::Lossless,
                    to: &TXT
                }
            ]
        })
    ]
};


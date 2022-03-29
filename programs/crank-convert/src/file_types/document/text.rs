use crate::conversions::simple_copy;
use crate::conversions::conversions::{ConversionQuality, ConversionStep, Conversion};
use crate::file_types::file_type::FileType;
use crate::formats::data_format::{empty_bytes, initialize_bytes};
use phf::{phf_set};
use lazy_static::lazy_static;
use crate::file_types::document::html::HTML;

pub const TXT: FileType = FileType {
    name: "TXT",
    extensions: phf_set!["txt", "text"],
    initialize: initialize_bytes,
    empty: empty_bytes,
    conversions: [
        (&HTML, Conversion {
            steps: *[
                ConversionStep {
                    execute: simple_copy,
                    quality: ConversionQuality::Lossless,
                    to: &HTML
                }
            ]
        })
    ]
};
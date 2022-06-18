use crate::conversion::{Conversion, ConversionChain};
use crate::conversions::basic::identity::identity;
use crate::file_data::FileData;
use crate::file_types::crank_file_type::CrankFileType;
use conversion_types::ConversionQuality;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TXT: CrankFileType = CrankFileType {
        name: "Text",
        extensions: vec!["txt", "text"],
        conversions: HashMap::from([(
            "RTF",
            vec![Conversion {
                conversion: identity,
                quality: ConversionQuality::default()
            }]
        )])
    };
}

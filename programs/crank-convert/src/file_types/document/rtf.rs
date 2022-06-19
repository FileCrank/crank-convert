use crate::conversion::{Conversion, ConversionChain};
use crate::conversions::basic::identity::identity;
use crate::file_types::crank_file_type::CrankFileType;
use conversion_types::ConversionQuality;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref RTF: CrankFileType = CrankFileType {
        name: "RTF",
        extensions: vec!["rtf"],
        conversions: HashMap::from([(
            "TXT",
            vec![Conversion {
                conversion: identity,
                quality: ConversionQuality {
                    formatting: false,
                    ..Default::default()
                }
            }]
        )])
    };
}

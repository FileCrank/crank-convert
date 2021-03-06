use crate::conversions::basic::identity::identity_conversion;
use crate::conversions::Conversion;
use crate::file_types::CrankFileType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TXT: CrankFileType = CrankFileType {
        name: "TXT",
        extensions: vec!["txt", "text"],
        conversions: HashMap::from([(
            "RTF",
            vec![Conversion {
                conversion: identity_conversion,
                quality: Default::default()
            }]
        )])
    };
}

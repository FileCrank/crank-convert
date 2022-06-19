use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::conversions::basic::identity::identity_conversion;
use crate::conversions::Conversion;
use crate::file_types::CrankFile;

lazy_static! {
    pub static ref TXT: CrankFile = CrankFile {
        name: "TXT",
        extensions: vec!["txt", "text"],
        conversions: HashMap::from([
            ("RTF", vec![
                Conversion {
                    conversion: identity_conversion,
                    quality: Default::default()
                }
            ])
        ])
    };
}


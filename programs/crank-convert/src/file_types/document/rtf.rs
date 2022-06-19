use crate::conversions::basic::identity::identity_conversion;
use crate::conversions::Conversion;
use crate::file_types::CrankFileType;
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
                conversion: identity_conversion,
                quality: ConversionQuality {
                    formatting: false,
                    ..Default::default()
                }
            }]
        )])
    };
}

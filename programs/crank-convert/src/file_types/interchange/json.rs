use lazy_static::lazy_static;
use crate::file_types::CrankFileType;
use crate::conversions::Conversion;
use conversion_types::ConversionQuality;
use crate::conversions::json::json_to_csv::json_to_csv;
use std::collections::HashMap;

lazy_static! {
    pub static ref JSON: CrankFileType = CrankFileType {
        name: "JSON",
        extensions: vec!["json"],
        conversions: HashMap::from([
            ("CSV", vec![
                Conversion {
                    conversion: json_to_csv,
                    quality: ConversionQuality {
                        streamability: false,
                        ..Default::default()
                    }
                }
            ])
        ])
    };
}
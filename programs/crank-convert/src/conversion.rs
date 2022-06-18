use crate::file_data::FileData;
use conversion_types::ConversionQuality;
use std::fmt::{Debug, Formatter};

pub struct Conversion {
    pub conversion: fn(&mut FileData) -> &mut FileData,
    pub quality: ConversionQuality,
}

impl Debug for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Conversion with quality {}",
            self.quality.quality()
        ))
    }
}

pub type ConversionChain = Vec<Conversion>;

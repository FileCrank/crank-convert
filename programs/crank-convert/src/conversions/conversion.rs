use crate::{DataHolder};
use anyhow::Result;
use conversion_types::ConversionQuality;
use std::fmt::{Debug, Display, Formatter};

pub struct Conversion {
    pub conversion: fn(&mut DataHolder) -> Result<()>,
    pub quality: ConversionQuality,
}

fn fmt_conversion(conv: &'_ Conversion, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
        "Conversion with quality {}",
        conv.quality.quality()
    ))
}

impl Display for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_conversion(self, f)
    }
}

impl Debug for Conversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt_conversion(self, f)
    }
}

pub type ConversionChain = Vec<Conversion>;

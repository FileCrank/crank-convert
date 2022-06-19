use crate::file_data::FileData;
use crate::CrankResult;
use conversion_types::ConversionQuality;
use std::fmt::{Debug, Formatter};

pub struct Conversion {
    pub conversion: fn(&mut FileData) -> CrankResult<()>,
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

/// Go through and execute all of the steps of a conversion chain
pub fn execute_chain(data: &mut FileData, chain: &'static ConversionChain) -> CrankResult<()> {
    for conv in chain {
       (conv.conversion)(data)?;
    }
    Ok(())
}
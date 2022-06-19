use conversion_types::ConversionQuality;
use crate::{CrankResult, DataHolder};

pub struct Conversion {
    pub conversion: fn(&mut DataHolder) -> CrankResult<()>,
    pub quality: ConversionQuality
}

pub type ConversionChain = Vec<Conversion>;

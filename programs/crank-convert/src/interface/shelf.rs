use std::fs::File;
use crate::conversions::{Conversion, CONVERSIONS, execute_conversion, get_conversion};
use crate::errors::ConversionError::UnsupportedConversionError;
use crate::errors::{ConversionError, Result};
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;
use crate::interface::conf::{parse_opt_file_type, parse_validate_opts, ConversionConf};
use crate::opts::OptFileType;
use crate::Opts;

fn convert_holder(mut conv_conf: ConversionConf) -> Result<Box<DataFormat>> {
    let mut conv = get_conversion(conv_conf.from, conv_conf.to)?;
    let format_data: DataFormat = (conv_conf.from.initialize)(&mut conv_conf.holder);
    conv.execute(&mut Box::new(format_data))
}

pub fn supported_conversions(oft: &OptFileType) -> Result<Vec<&'static str>> {
    let file_type: &FileType = parse_opt_file_type(oft)?;
    Ok(file_type.conversions.keys().map(|x| x.name).collect())
}

pub fn convert(opts: Opts) -> Result<Box<DataFormat>> {
    let conv_conf = parse_validate_opts(opts)?;
    convert_holder(conv_conf)
}

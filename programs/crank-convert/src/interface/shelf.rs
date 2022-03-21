use std::ops::Deref;
use crate::conversions::execute_conversion;
use crate::errors::ConversionError::{InvalidConfigurationError, UnsupportedConversionError};
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;
use crate::formats::data_holder::DataHolder;
use crate::errors::Result;
use crate::interface::conf::{ConversionConf, parse_opt_file_type, parse_validate_opts};
use crate::Opts;
use crate::opts::OptFileType;

fn convert_holder(mut conv_conf: ConversionConf) -> Result<Box<DataFormat>> {
    let format_data: DataFormat = (conv_conf.from.initialize)(&mut conv_conf.holder);
    if let Some(conversion) = conv_conf.from.conversions.get(&conv_conf.to) {
        execute_conversion(&mut Box::new(format_data), conv_conf.to, *conversion)
    } else {
        Err(UnsupportedConversionError)
    }
}

pub fn supported_conversions(oft: OptFileType) -> Result<Vec<&'static str>> {
    let file_type: &FileType = parse_opt_file_type(oft)?;
    Ok(file_type
        .conversions
        .keys()
        .map(|x| x.name)
        .collect())
}


pub fn convert(opts: Opts) -> Result<Box<DataFormat>> {
    let conv_conf = parse_validate_opts(opts)?;
    convert_holder(conv_conf)
}
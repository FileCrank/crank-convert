use crate::conversions::execute_conversion;
use crate::errors::ConversionError::UnsupportedConversionError;
use crate::errors::Result;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;
use crate::interface::conf::{parse_opt_file_type, parse_validate_opts, ConversionConf};
use crate::opts::OptFileType;
use crate::Opts;

fn convert_holder(mut conv_conf: ConversionConf) -> Result<Box<DataFormat>> {
    let format_data: DataFormat = (conv_conf.from.initialize)(&mut conv_conf.holder);
    if let Some(conversion) = conv_conf.from.conversions.get(&conv_conf.to) {
        execute_conversion(&mut Box::new(format_data), conv_conf.to, *conversion)
    } else {
        Err(UnsupportedConversionError)
    }
}

pub fn supported_conversions(oft: &OptFileType) -> Result<Vec<&'static str>> {
    let file_type: &FileType = parse_opt_file_type(oft)?;
    Ok(file_type.conversions.keys().map(|x| x.name).collect())
}

pub fn convert(opts: Opts) -> Result<Box<DataFormat>> {
    let conv_conf = parse_validate_opts(opts)?;
    convert_holder(conv_conf)
}

use crate::conversions::ConversionStep;
use crate::errors::Result;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;

pub fn execute_conversion(
    from_data: &mut Box<DataFormat>,
    to_file_type: &FileType,
    step: ConversionStep,
) -> Result<Box<DataFormat>> {
    let mut empty: Box<DataFormat> = Box::new((to_file_type.empty)());
    match step.execute(from_data, &mut empty) {
        Ok(bytes_converted) => {
            log::debug!("Successfully transferred {} bytes over", bytes_converted);
            Ok(empty)
        }
        Err(e) => Err(e),
    }
}

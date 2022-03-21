use crate::errors::Result;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;

pub type Conversion = fn(&mut Box<DataFormat>, &mut Box<DataFormat>) -> Result<usize>;

pub fn execute_conversion(
    from_data: &mut Box<DataFormat>,
    to_file_type: &FileType,
    conversion: Conversion,
) -> Result<Box<DataFormat>> {
    let mut empty: Box<DataFormat> = Box::new((to_file_type.empty)());
    match conversion(from_data, &mut empty) {
        Ok(bytes_converted) => {
            log::debug!("Successfully transferred {} bytes over", bytes_converted);
            Ok(empty)
        }
        Err(e) => Err(e),
    }
}

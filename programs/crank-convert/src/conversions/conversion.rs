use crate::errors::Result;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;

pub type Conversion<F, T> = Box<fn(&mut Box<F>, &mut Box<T>) -> Result<usize>>;

pub fn execute_conversion<F, T>(from_data: &mut Box<F>,
                                to_file_type: FileType<T>,
                                conversion: Conversion<F, T>) -> Result<Box<T>>
where F: DataFormat, T: DataFormat {
    let mut empty: Box<T> = Box::new((to_file_type.empty)());
    match conversion(from_data, &mut empty) {
        Ok(bytes_converted) => {
            log::debug!("Successfully transferred {} bytes over", bytes_converted);
            Ok(empty)
        }
        Err(e) => Err(e)
    }
}

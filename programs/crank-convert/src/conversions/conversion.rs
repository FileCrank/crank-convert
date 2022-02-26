use crate::formats::data_format::DataFormat;
use crate::errors::Result;

pub type Conversion<F, T> = Box<fn(&mut Box<F>, &mut Box<T>) -> Result<usize>>;

pub fn execute_conversion<F, T>(from: &mut Box<F>,
                                conversion: Conversion<F, T>) -> Result<Box<T>>{
    let mut empty: Box<T> = Box::new(T::default());
    match conversion(from, &mut empty) {
        Ok(bytes_converted) => {
            log::debug!("Successfully transferred {} bytes over", bytes_converted);
            Ok(empty)
        }
        Err(e) => e
    }
}

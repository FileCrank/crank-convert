use std::fs::File;
use phf::phf_map;
use convert_proc::conversions;
use crate::conversions::execute_conversion;
use crate::errors::ConversionError::UnsupportedConversionError;
use crate::file_types::file_type::FileType;
use crate::formats::data_format::DataFormat;
use crate::errors::Result;
use crate::formats::data_holder::DataHolder;


/// Instantiate conversions with map syntax
macro_rules! old_conversions {
     ($($to: ident => $how: ident),* $(,)?) => {
         phf_map! {
             $(&$to => $how as crate::conversions::Conversion),*
         }
    };
    ($($source: ident => {($converts_to: tt)}),* $(,)?) => {
        phf_map! {
            $($source => conversions!($converts_to),)*
        }
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq)]
pub enum ConversionQuality {
    Lossless,
    Lossy
}

pub struct ConversionStep {
    pub execute: fn(&mut Box<DataFormat>, &mut Box<DataFormat>) -> Result<usize>,
    pub quality: ConversionQuality,
    pub to: &'static FileType,
}

pub struct Conversion {
    pub steps: [ConversionStep],
}

impl Conversion {
    pub fn execute(&mut self, data: &mut Box<DataFormat>) -> Result<Box<DataFormat>> {
        let mut res: Box<DataFormat>;
        let mut first: bool = true;

        for step in self.steps {
            let from_data: &mut Box<DataFormat>;
            if first {
                from_data = data;
                first = false;
            } else {
               from_data = &mut res;
            }

           match execute_conversion(from_data, step.to, step) {
               Ok(c) => res = c,
               Err(e) => return Err(e)
           }
        }

        Ok(curr)
    }
}

/// A HashMap mapping
pub type ConversionMap = phf::Map<&'static str, &'static Conversion>;

pub const CONVERSIONS: phf::Map<&'static str, ConversionMap> = include!("gen/conversion_mappings.rs");
pub const FILE_TYPE_NAMES: phf::Map<&'static str, &'static FileType> = include!("gen/file_type_name_mappings.rs");
pub const FILE_TYPE_EXTENSIONS: phf::Map<&'static str, &'static FileType> = include!("gen/file_type_extension_mappings.rs");

pub fn get_conversion(from: &'static FileType,
                      to: &'static FileType) -> Result<&Conversion> {
    if let Some(from_conversions) = CONVERSIONS.get(from.name) {
        match from_conversions.get(to.name) {
            Some(conv) => Ok(conv),
            None => Err(UnsupportedConversionError)
        }
    } else {
        unreachable!("Provided impossible file type");
    }
}
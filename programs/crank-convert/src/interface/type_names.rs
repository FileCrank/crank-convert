use crate::file_types::document::html::HTML;
use crate::file_types::document::text::TXT;
use crate::file_types::file_type::FileType;
use lazy_static::lazy_static;
use std::collections::HashMap;

// TODO: generate this stuff with a build.rs
/*
lazy_static! {

    pub static ref FILE_TYPE_NAMES: HashMap<&'static str, &'static FileType> = {
        let mut file_type_name_map: HashMap<&'static str, &'static FileType> =
            HashMap::new();
        for file_type in CONVERSIONS.keys() {
            file_type_name_map.insert(file_type.name, *file_type);
        }
        file_type_name_map
    };

    pub static ref FILE_TYPE_EXTENSIONS: HashMap<&'static str, &'static FileType> = {
        let mut file_type_extension_map: HashMap<&'static str, &'static FileType> =
            HashMap::new();
        for file_type in CONVERSIONS.keys() {
            for extension in file_type.extensions {
                file_type_extension_map.insert(extension, *file_type);
            }
        }
        file_type_extension_map
    };
}


 */
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::file_types::document::html::HTML;
use crate::file_types::document::text::TXT;
use crate::file_types::file_type::FileType;

lazy_static! {
    pub static ref FILE_TYPES: Vec<&'static FileType<'static>> = vec![
        TXT.deref(),
        HTML.deref()
    ];

    pub static ref FILE_TYPE_NAMES: HashMap<&'static str, &'static FileType<'static>> = {
        let mut file_type_name_map: HashMap<&'static str, &'static FileType<'static>> = HashMap::new();
        for file_type_ref in FILE_TYPES.deref() {
            let file_type = file_type_ref.deref();
            file_type_name_map.insert(file_type.name, file_type_ref);
        }
        file_type_name_map
    };

    pub static ref FILE_TYPE_EXTENSIONS: HashMap<&'static str, &'static FileType<'static>> = {
        let mut file_type_extension_map: HashMap<&'static str, &'static FileType<'static>> = HashMap::new();
        for file_type_ref in FILE_TYPES.deref() {
            let file_type = file_type_ref.deref();
            for extension in file_type.extensions {
                file_type_extension_map.insert(extension, file_type_ref);
            }
        }
        file_type_extension_map
    };
}
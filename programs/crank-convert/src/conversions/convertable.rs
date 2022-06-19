use crate::file_types::document::{rtf::RTF, txt::TXT};
use crate::file_types::CrankFileType;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CONVERTABLE: Vec<&'static CrankFileType> = vec![
        &TXT,
        &RTF
    ];

    pub static ref EXTENSION_MAPPINGS: HashMap<&'static str, &'static CrankFileType> = {
        let mut extension_map = HashMap::new();

        // Have to do this weird &* because of the strange type of lazy static values -
        // we need to dereference it to evaluate it, and then take care not to move it
        for file_type in &*CONVERTABLE {
            for extension in &file_type.extensions {
                extension_map.insert(*extension, *file_type);
            }
        }

        extension_map
    };
}

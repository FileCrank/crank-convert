// Debugging code, formatters, etc.

use crate::file_types::file_type::FileType;
use std::fmt::{Debug, Formatter};

impl Debug for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("File type {}", self.name))
    }
}

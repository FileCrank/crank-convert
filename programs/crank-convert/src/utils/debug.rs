// Debugging code, formatters, etc.

use std::fmt::{Debug, Formatter};
use crate::file_types::file_type::FileType;

impl Debug for FileType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("File type {}", self.name))
    }
}
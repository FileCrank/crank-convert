use crate::conversions::{
    convertable::Convertable,
    documents::{
        html::HTML,
        rtf::RTF
    }
};
extern crate convert_proc;
use convert_proc::Convertable;
use crate::conversions::common::simple_copy::simple_copy;

#[derive(Convertable)]
#[convertable(
    name = "Text",
    extension = "txt",
    extension = "text")
]
#[converts(
    HTML => simple_copy,
    TXT => simple_copy
)]
pub struct TXT {}

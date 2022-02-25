use crate::conversions::{
    convertable::Convertable,
    documents::{html::HTML, txt::TXT},
};
use convert_proc::Convertable;
use lazy_static::lazy_static;
use crate::errors::Result;
use crate::conversions::common::simple_copy::simple_copy;

#[derive(Convertable)]
#[convertable(name = "Rich Text Format", extension = "rtf")]
#[converts(
    HTML => simple_copy,
    TXT @ convert_to_txt
)]
pub struct RTF {}

impl RTF {
    fn convert_to_txt(&self, dest: &TXT) -> Result<usize> {
        simple_copy(self, dest)
    }
}

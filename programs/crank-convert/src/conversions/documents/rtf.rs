use crate::conversions::{
    convertable::Convertable,
    documents::{html::HTML, txt::TXT},
};
use convert_proc::Convertable;
use lazy_static::lazy_static;

#[derive(Convertable)]
#[convertable(name = "Rich Text Format", extension = "rtf")]
pub struct RTF {}

impl RTF {
    fn convert_to_html(&self) {}

    fn convert_to_txt(&self) {}
}

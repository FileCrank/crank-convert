use crate::conversions::convertable::Convertable;
extern crate convert_proc;
use convert_proc::Convertable;

#[derive(Convertable)]
#[convertable(
    name="Text",
    extension="txt",
    extension="text"
)]
#[converts(to="HTML", handler="convert_to_html")]
pub struct TXT {
    pub test: String,
}



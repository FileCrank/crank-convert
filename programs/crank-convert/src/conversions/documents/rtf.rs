use convert_proc::Convertable;
use crate::conversions::documents::{
    txt::TXT,
    html::HTML,
};

#[derive(Convertable)]
#[convertable(
    name="Rich Text Format",
    extension="rtf"
)]
#[converts(to="TXT", handler="convert_to_txt")]
#[converts(to="HTML", handler="convert_to_html")]
pub struct RTF {

}
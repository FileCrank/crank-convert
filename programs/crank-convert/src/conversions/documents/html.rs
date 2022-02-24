use crate::conversions::convertable::Convertable;

use convert_proc::Convertable;
use crate::conversions::documents::txt::TXT;

#[derive(Convertable)]
#[convertable(
    name="HTML",
    extension="html",
    extension="htm"
)]
#[converts(to="TXT", handler="convert_to_txt")]
pub struct HTML {}

impl HTML {
    fn convert_to_txt(&self) {
        todo!()
    }
}
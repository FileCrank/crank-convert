use crate::conversions::convertable::Convertable;

use crate::conversions::documents::txt::TXT;
use convert_proc::Convertable;

#[derive(Convertable)]
#[convertable(name = "HTML", extension = "html", extension = "htm")]
pub struct HTML {}

impl HTML {
    fn convert_to_txt(&self) {
        todo!()
    }
}

use crate::conversions::convertable::Convertable;

use convert_proc::Convertable;

#[derive(Convertable)]
#[convertable(
    name="HTML",
    extensions=["html"],
    conversions={
        TXT: convert_txt
    }
)]
pub struct HTML {

}
use crate::conversions::convertable::Convertable;
extern crate convert_proc;
use convert_proc::Convertable;

#[derive(Convertable)]
#[convertable(name="TXT")]
pub struct Txt {
    pub test: String,
}



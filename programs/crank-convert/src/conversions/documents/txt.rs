use std::fmt::Write;
use std::io::{BufReader, Read};
use std::ops::Deref;
use crate::conversions::{
    convertable::Convertable,
    documents::{
        html::HTML,
        rtf::RTF
    }
};
extern crate convert_proc;
use convert_proc::Convertable;
use crate::conversions::common::simple_convertable::SimpleConvertable;
use crate::conversions::common::simple_copy::simple_copy;

#[derive(Convertable)]
#[convertable(
    name = "Text",
    extension = "txt",
    extension = "text"
)]
#[converts(
    HTML => simple_copy,
    TXT => simple_copy
)]
pub struct TXT<T> {
    inner: SimpleConvertable<T>
}

impl<T> Deref for TXT<T> {
    type Target = SimpleConvertable<T>;

    fn deref(&self) -> &Self::Target {
        *self.inner
    }
}
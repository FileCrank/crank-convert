use std::io::{Read, Write};
use serde::Serialize;
use serde_json::Value;

pub struct JSONFormat {
    parsed: Option<Value>,
    raw: Option<String>
}

macro_rules! json_match {
    ($n: ident, $var: ident, $parsed: block, $raw: block) => {
        match $n {
            JSONFormat {
                parsed: Some($var),
                raw: None
            } => $parsed,
            JSONFormat {
                parsed: None,
                raw: Some($var)
            } => $raw,
            _ => unreachable!()
        }
    }
}

impl Read for JSONFormat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        json_match!(self, i, {
            // If i is a parsed value
            match i.serialize(buf) {
                Ok(_) => todo!(),
                Err(e) => todo!()
            }
        }, {
            // if I is a string
            buf.write(i)
        })
    }
}

impl Write for JSONFormat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        unreachable!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unreachable!()
    }
}
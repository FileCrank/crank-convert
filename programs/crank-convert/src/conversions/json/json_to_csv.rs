use crate::holder_utils::read_holder_to_string;
use crate::{DataHolder};
use csv::Writer;
use serde_json::{from_reader, to_string, Value};
use std::io::Read;
use anyhow::Result;

fn json_value_to_csv(value: Value, writer: &mut Writer<Vec<u8>>) -> Result<()> {
    match value {
        Value::Array(arr) => {
            for val in arr {
                json_value_to_csv(val, writer)?;
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj.into_iter() {
                writer.write_record([key, to_string(&val)?])?;
            }
        }
        _ => {
            // With anything else, just write a string value to the first cell
            writer.write_field(to_string(&value)?)?;
        }
    }
    Ok(())
}

/// Convert a JSON file to a CSV file
pub fn json_to_csv(holder: &mut DataHolder) -> Result<()> {
    // We have to parse the JSON, so read the whole thing in
    let value = from_reader(holder)?;

    let mut new_data = Vec::new();
    let mut writer = Writer::from_writer(new_data);
    json_value_to_csv(value, &mut writer)?;
    writer.flush()?;

    *holder = DataHolder::Bytes(writer.into_inner()?);

    Ok(())
}

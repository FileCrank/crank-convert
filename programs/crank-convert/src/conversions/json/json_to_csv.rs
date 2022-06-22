use std::io::Read;
use crate::{CrankResult, DataHolder};
use crate::holder_utils::read_holder_to_string;
use serde_json::{from_reader, Value};
use csv::Writer;
use crate::conversions::json::json_utils::get_top_level_keys;

fn json_value_to_csv(value: Value, writer: &mut Writer<Vec<u8>>) {
    match value {
        Value::Array(arr) => {
            for val in arr {
                json_value_to_csv(val, writer)
            }
        },
        Value::Object(_) => {}
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(_) => {}
        Value::String(_) => {}
    }
}

/// Convert a JSON file to a CSV file
pub fn json_to_csv(holder: &mut DataHolder) -> CrankResult<()> {
    // We have to parse the JSON, so read the whole thing in
    let value = from_reader(holder)?;

    let mut new_data: Vec<u8> = Vec::new();
    let mut writer = Writer::from_writer(new_data);
    let keys = get_top_level_keys(value);
    writer.write_record(keys)?;

    // TODO: write the code that will take this list of keys, and turn it into a CSV

    Ok(())
}
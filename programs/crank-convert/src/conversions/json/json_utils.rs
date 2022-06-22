use std::collections::HashSet;
use serde_json::Value;

/// Get all the keys that appear in the top level of a JSON value
pub fn get_top_level_keys(value: Value) -> HashSet<String> {
    match value {
        Value::Array(a) => {
            let mut key_set: HashSet<String> = HashSet::new();
            for val in a {
                let val_key_set = get_top_level_keys(val);
                key_set.extend(val_key_set);
            }
            key_set
        },
        Value::Object(o) => {
            let key_vec: Vec<String> = o.keys().collect();
            key_vec.into()
        }
        _ => HashSet::new()
    }
}
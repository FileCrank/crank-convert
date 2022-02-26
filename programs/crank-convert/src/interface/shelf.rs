use std::collections::HashMap;
use crate::conversions::convertable::Convertable;
use crate::conversions::documents::html::HTML;

fn test() {
    // TODO: This doesn't work! Need to do a model instead where there are functions or
    // TODO: closures inside this array, which can create a new HTML instance
    let available_conversions: HashMap<&'static str, Box<dyn Convertable>> =
        HashMap::from([
            ("test", HTML)
        ]);
}
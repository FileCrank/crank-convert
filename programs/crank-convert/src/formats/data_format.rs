use std::collections::HashSet;

pub struct DataFormat {
    pub name: &'static str,
    pub extensions: HashSet<&'static str>,
}


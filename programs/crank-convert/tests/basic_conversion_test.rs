mod utils;

#[cfg(test)]
pub mod basic_conversion_test {
    use std::env;
    use crank_convert::file_types::document::html::HTML;
    use crank_convert::file_types::document::text::TXT;
    use crank_convert::interface::shelf::convert;
    use std::ops::Deref;
    use std::path::Path;
    use crank_convert::opts::{OptFileType, Opts};

    #[test]
    pub fn test_basic_conversion() {
        let out_file = OptFileType::Name("out.html".to_string());
        let opts = Opts::from_file(
            Path::new("tests/test_in/basic.txt"),
            &out_file);
        let res = convert(opts).unwrap();
        println!("Here!");
    }
}

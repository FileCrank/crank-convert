macro_rules! test_file_path {
    ($n: literal) => {
        format!("tests/test_files/{}", $n)
    };
}

#[cfg(test)]
pub mod basic_conversion_test {
    use crank_convert::file_types::document::rtf::RTF;
    use crank_convert::file_types::document::txt::TXT;
    use crank_convert::{convert, FileData};
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_basic_conversion() {
        let basic_text_file = File::open(test_file_path!("basic_test.txt")).unwrap();
        let mut buf = BufReader::new(basic_text_file);
        let mut data = FileData {
            data: buf,
            file_type: &TXT,
        };
        let res = convert(&mut data, &RTF).unwrap();
        let mut data = String::new();
        res.data.read_to_string(&mut data).unwrap();
        assert!(data.as_str() == "Basic Text File");
    }
}

mod test_macros;

#[cfg(test)]
pub mod basic_conversion_test {
    use crate::test_file;
    use crank_convert::convert;
    use crank_convert::data_holder::DataHolder;
    use crank_convert::file_types::document::rtf::RTF;
    use crank_convert::file_types::document::txt::TXT;
    use crank_convert::opts::CrankOpts;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_basic_txt_to_rtf() {
        let test_file = File::open(test_file!("basic_test.txt")).unwrap();
        let mut holder = DataHolder::File(test_file);
        convert(
            &mut holder,
            CrankOpts {
                from: &TXT,
                to: &RTF,
            },
        )
        .unwrap();
        match holder {
            DataHolder::File(mut f) => {
                let mut data_str = String::new();
                f.read_to_string(&mut data_str).unwrap();
                assert_eq!(data_str, "Basic Text File");
            }
            DataHolder::Bytes(_) => unreachable!(),
        }
    }
}

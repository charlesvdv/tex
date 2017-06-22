extern crate tex;

mod tex_format;
mod tex_compare;

use tex_compare::test_parser;

macro_rules! generate_test {
    ($test_name:ident) => (
        #[test]
        fn $test_name() {
            test_parser(stringify!($test_name));
        }
    )
}

generate_test!(simple);
generate_test!(simple_multiline);
generate_test!(comment);
generate_test!(simple_catcode);
generate_test!(catcodes_groups);

//! Format the output of this TeX parser into a string which
//! is TeX compatible.

use tex::parser::TeXToken;

pub fn format(parser_output: &Vec<TeXToken>) -> String {
    let mut out = String::new();
    for elem in parser_output {
        match elem {
            &TeXToken::Text(ref v) => {
                out.push_str(&v);
            }
            _ => continue,
        }
    }
    out.push_str("\\bye");
    out
}

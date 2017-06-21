//! Format the output of this TeX parser into a string which
//! is TeX compatible.

use tex::parser::TeXToken;

pub fn format(parser_output: &Vec<TeXToken>) -> String {
    let mut out = String::new();
    for elem in parser_output {
        match elem {
            &TeXToken::Text(ref v) => {
                out.push_str(&escape_str(v));
            }
            _ => continue,
        }
    }
    out.push_str("\n\\bye");
    out
}

fn escape_str(seq: &str) -> String {
    let escaped_ch = [
        ('&', "\\&"),
        ('%', "\\%"),
        ('$', "\\$"),
        ('#', "\\#"),
        ('_', "\\_"),
        ('{', "$\\{$"),
        ('}', "$\\}$"),
        ('~', "\\~"),
        // TODO
        ('^', ""),
        ('\\', "\\backslash"),
    ];
    let mut output = String::new();

    for ch in seq.chars() {
        let mut escaped = false;
        for esc_ch in escaped_ch.iter() {
            if esc_ch.0 == ch {
                escaped = true;
                output.push_str(esc_ch.1);
            }
        }
        if !escaped {
            output.push(ch);
        }
    }
    output
}

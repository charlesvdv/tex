use common::tex::parser::*;

// Format ouptput from parser.
pub fn format(input: Vec<ParserElem>) -> String {
    let mut output = String::new();
    for elem in input {
        match elem {
            ParserElem::Text(v) => output.push_str(&escape_str(&v)),
            _ => continue,
        }
    }
    output.push_str("\\bye");
    output
}

fn escape_str(seq: &str) -> String {
    let escaped_ch = [('&', "\\&"),
                      ('%', "\\%"),
                      ('$', "\\$"),
                      ('#', "\\#"),
                      ('_', "\\_"),
                      ('{', "$\\{$"),
                      ('}', "$\\}$"),
                      ('~', "\\~"),
                      // TODO
                      ('^', ""),
                      ('\\', "\\backslash")];
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

macro_rules! catcode_test {
    ($name:ident, $code:expr) => (
        #[inline]
        pub fn $name(&self, ch: char) -> bool {
            for c in &self.codes[$code as usize] {
                if c == &ch {
                    return true;
                }
            }
            return false;
        }
    )
}

#[derive(Debug)]
pub struct Catcodes {
    codes: [Vec<char>; 16],
}

impl Catcodes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_catcode(&mut self, index: i8, value: char) {
        assert!(index > 0 && index < 16);

        // Check if the code is used by others catcodes.
        for code in &mut self.codes {
            code.retain(|&x| x != value);
        }

        // Update the codes.
        self.codes[index as usize].push(value);
    }

    catcode_test!(is_control_sequence, 0);
    catcode_test!(is_begin_group, 1);
    catcode_test!(is_end_group, 2);
    catcode_test!(is_math_shift, 3);
    catcode_test!(is_alignement_tab, 4);
    catcode_test!(is_end_of_line, 5);
    catcode_test!(is_macro_param, 6);
    catcode_test!(is_superscript, 7);
    catcode_test!(is_subscript, 8);
    catcode_test!(is_ignored_char, 9);
    catcode_test!(is_space, 10);
    catcode_test!(is_letter, 11);

    // code 12
    #[inline]
    pub fn is_other_character(&self, ch: char) -> bool {
        if self.codes[12].contains(&ch) {
            return true;
        }

        for code in &self.codes {
            if code.contains(&ch) {
                return false;
            }
        }
        true
    }

    catcode_test!(is_active, 13);
    catcode_test!(is_comment, 14);
    catcode_test!(is_invalid, 15);

    pub fn is_escaped_char(&self, ch: char) -> bool {
        ['&', '%', '$', '#', '_', '{', '}', '~', '^', '\\'].contains(&ch)
    }
}

impl Default for Catcodes {
    fn default() -> Self {
        Catcodes {
            codes: [
                    vec!['\\'],
                    vec!['{'],
                    vec!['}'],
                    vec!['$'],
                    vec!['&'],
                    vec!['\n', 10 as char], // 10 is the Vertical Tab ASCII character
                    vec!['#'],
                    vec!['^'],
                    vec!['_'],
                    vec![0 as char], // ASCII null
                    vec![' ', 13 as char], // 13 is the Horizontal Tab ASCII character
                    generate_alpha(),
                    vec!['@'], // And others...
                    vec!['~'],
                    vec!['%'],
                    vec![127 as char], // 127 is the delete ASCII character
                ],
        }
    }
}

fn generate_alpha() -> Vec<char> {
    let mut alpha: Vec<_> = (0..26).map(|x| (x + 'a' as u8) as char).collect();
    alpha.append(&mut (0..26).map(|x| (x + 'A' as u8) as char).collect());
    alpha
}

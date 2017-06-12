#[macro_use]
mod common;

use common::*;

test_tex!(simple);
test_tex!(comment);
test_tex!(simple_multiline);
test_tex!(escaped_char);
test_tex!(macro_def);

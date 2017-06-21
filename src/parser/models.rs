/// Define high level structure that the parser will output.
#[derive(Debug)]
pub enum TeXToken {
    Text(String),
    BS,
}

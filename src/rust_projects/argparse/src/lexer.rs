//TODO: Break out parser into lexer and parser
pub mod validators;

pub enum Token {
    Flag(String),
    ArgName(String),
    Value(String),
    InvalidArgName,
}

pub struct Lexer {
    args: Vec<String>,
    cur: usize,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.args.len() {
            return None;
        }

        //let arg: &str = &self.args[self.cur];

        None
    }
}

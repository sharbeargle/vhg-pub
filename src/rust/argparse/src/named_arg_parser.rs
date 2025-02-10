use std::{error::Error, fmt};

//TODO: Break out named arg parsing into this module

pub enum NamedArgToken {
    ArgName(String),
    Value(String),
    InvalidArgName,
    InvalidValue,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NamedArgLexerError {
    DashesError,
}

impl Error for NamedArgLexerError {}

impl fmt::Display for NamedArgLexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DashesError => write!(f, "Named arg has invalid number of dashes"),
        }
    }
}

pub struct NamedArgLexer {
    arg: Vec<char>,
    cur: usize,
}

pub fn new(arg: String) -> NamedArgLexer {
    NamedArgLexer {
        arg: arg.chars().collect(),
        cur: 0,
    }
}

impl Iterator for NamedArgLexer {
    type Item = NamedArgToken;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.arg.len() {
            return None;
        }

        None
    }
}

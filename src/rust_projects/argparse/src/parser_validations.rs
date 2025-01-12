use std::{collections::hash_set::HashSet, error::Error, fmt};

/// Helper functions for validating arguments

#[derive(Debug)]
pub enum ParserValidationError {
    FlagFormatError,
    InvalidFlag,
    DuplicateFlag,
}

impl Error for ParserValidationError {}

impl fmt::Display for ParserValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FlagFormatError => write!(f, "Flag has invalid format"),
            Self::InvalidFlag => write!(f, "Flag has invalid character"),
            Self::DuplicateFlag => write!(f, "Flag was specified multiple times"),
        }
    }
}

/// Validate the flag arg and return whether or not flag is valid
pub fn validate_flag_format(flag: &str) -> Result<(), ParserValidationError> {
    let flag_chars: Vec<char> = flag.chars().into_iter().collect();

    // Validate length is at least two (dash + flag)
    if flag_chars.len() < 2 {
        return Err(ParserValidationError::FlagFormatError);
    }

    // Validate that it starts with a single dash
    if flag_chars[0] != '-' {
        return Err(ParserValidationError::FlagFormatError);
    }

    // Validate remaining flag chars are alphanumeric and unique
    let mut seen: HashSet<char> = HashSet::new();
    for c in &flag_chars[1..] {
        if !c.is_alphanumeric() {
            return Err(ParserValidationError::InvalidFlag);
        }

        if seen.contains(c) {
            return Err(ParserValidationError::DuplicateFlag);
        }

        seen.insert(*c);
    }

    Ok(())
}

use std::{collections::hash_set::HashSet, error::Error, fmt};

/// Helper functions for validating arguments

#[derive(Debug, PartialEq, Eq)]
pub enum ParserValidationError {
    FlagFormatError,
    InvalidFlag,
    InvalidLongFlagName,
    DuplicateFlag,
    NamedArgFormatError,
    InvalidNamedArgName,
    NamedArgValueFormatError,
    InvalidNamedArgValue,
    InvalidPositionalArgumentFormat,
}

impl Error for ParserValidationError {}

impl fmt::Display for ParserValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FlagFormatError => write!(f, "Flag has invalid format"),
            Self::InvalidFlag => write!(f, "Flag has invalid character"),
            Self::DuplicateFlag => write!(f, "Flag was specified multiple times"),
            Self::NamedArgFormatError => write!(f, "Named Arg has invalid format"),
            Self::InvalidNamedArgName => write!(f, "Named Arg name has invalid characters"),
            Self::InvalidLongFlagName => write!(f, "Long flag name has invalid characters"),
            Self::NamedArgValueFormatError => write!(f, "Named arg value missing end quotes"),
            Self::InvalidNamedArgValue => write!(f, "Named arg value contains invalid characters"),
            Self::InvalidPositionalArgumentFormat => {
                write!(f, "Positional arg value contains invalid characters")
            }
        }
    }
}

/// Verify that the argument name contains valid characters
fn is_valid_arg_name(name: &str) -> bool {
    let valid_non_alphanumerics: HashSet<char> = HashSet::from(['_', '-']);

    if name.len() < 1 {
        return false;
    }

    if !name.chars().nth(0).unwrap().is_alphanumeric() {
        return false;
    }

    for c in name.chars().collect::<Vec<char>>()[1..].into_iter() {
        if !(c.is_alphanumeric() || valid_non_alphanumerics.contains(c)) {
            return false;
        }
    }
    true
}

/// Validate the named arg
pub fn validate_named_arguments_format(
    named_arg: &str,
) -> Result<(String, String), ParserValidationError> {
    // Validate length is at least 3 (two dashes + string)
    if named_arg.len() < 3 {
        return Err(ParserValidationError::NamedArgFormatError);
    }

    // Validate has an equal and split the arg into name and value
    let (arg_name, mut arg_value): (String, String) = match named_arg[2..].split_once("=") {
        Some(arg_kv) => (arg_kv.0.to_owned(), arg_kv.1.to_owned()),
        None => {
            return Err(ParserValidationError::NamedArgFormatError);
        }
    };

    // Validate arg name
    if !is_valid_arg_name(&arg_name) {
        return Err(ParserValidationError::InvalidNamedArgName);
    }

    // Remove quotes if present
    if let Some(stripped_left) = arg_value.strip_prefix('"') {
        // Expect that suffix quote exists as well if prefix quote exists
        if let Some(stripped_right) = stripped_left.strip_suffix('"') {
            arg_value = stripped_right.to_owned();
        } else {
            return Err(ParserValidationError::NamedArgValueFormatError);
        }
    }

    // Validate value characters are valid chars
    for c in arg_value.chars() {
        if !c.is_alphanumeric() {
            return Err(ParserValidationError::NamedArgFormatError);
        }
    }

    Ok((arg_name, arg_value))
}

pub fn validate_long_flag_format(flag: &str) -> Result<String, ParserValidationError> {
    for c in flag.chars() {
        if !c.is_alphanumeric() {
            return Err(ParserValidationError::InvalidLongFlagName);
        }
    }

    Ok(flag.to_owned())
}

/// Validate the flag arg
pub fn validate_flag_format(flag: &str) -> Result<Vec<char>, ParserValidationError> {
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

    let flags: Vec<char> = seen.into_iter().collect();

    Ok(flags)
}

//TODO: Fix this function. It is not validating correctly
pub fn validate_positional_arguments_format(arg: &str) -> Result<String, ParserValidationError> {
    let mut arg_value = arg.to_owned();

    // Remove quotes if present
    if let Some(stripped_left) = arg_value.strip_prefix('"') {
        // Expect that suffix quote exists as well if prefix quote exists
        if let Some(stripped_right) = stripped_left.strip_suffix('"') {
            arg_value = stripped_right.to_owned();
        } else {
            return Err(ParserValidationError::InvalidPositionalArgumentFormat);
        }

        return Ok(arg_value);
    }

    for c in arg_value.chars() {
        if !c.is_alphanumeric() {
            return Err(ParserValidationError::InvalidPositionalArgumentFormat);
        }
    }

    Ok(arg_value)
}

#[cfg(test)]
mod tests {
    use crate::parser_validations::is_valid_arg_name;

    #[test]
    fn test_is_valid_arg_name() {
        assert_eq!(is_valid_arg_name("myarg"), true);
        assert_eq!(is_valid_arg_name("my_arg"), true);
        assert_eq!(is_valid_arg_name("_myarg"), false);
        assert_eq!(is_valid_arg_name("my arg"), false);
    }
}

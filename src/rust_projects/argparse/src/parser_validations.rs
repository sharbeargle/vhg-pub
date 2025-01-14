use std::{collections::hash_set::HashSet, error::Error, fmt};

/// Helper functions for validating arguments

#[derive(Debug)]
pub enum ParserValidationError {
    FlagFormatError,
    InvalidFlag,
    InvalidLongFlagName,
    DuplicateFlag,
    NamedArgFormatError,
    InvalidNamedArgName,
    NamedArgValueFormatError,
    InvalidNamedArgValue,
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
        }
    }
}

/// Validate the named arg
pub fn validate_named_arguments_format(
    named_arg: &str,
) -> Result<(String, String), ParserValidationError> {
    let arg_name: String;
    let mut arg_value: String;

    // TODO: Fix this. This length validation is wrong
    // Validate length is at least five (two dashes + string + = + string)
    if named_arg.len() < 5 {
        return Err(ParserValidationError::NamedArgFormatError);
    }

    // Validate has an equal
    match named_arg[2..].split_once("=") {
        Some(arg_kv) => {
            arg_name = arg_kv.0.to_owned();
            arg_value = arg_kv.1.to_owned();
        }
        None => {
            return Err(ParserValidationError::NamedArgFormatError);
        }
    }

    // Validate at the arg name has valid characters
    for c in arg_name.chars() {
        if !c.is_alphanumeric() {
            return Err(ParserValidationError::InvalidNamedArgName);
        }
    }

    // Process arg value with or without quotes
    if arg_value.starts_with('"') {
        arg_value = arg_value.strip_prefix('"').unwrap().to_owned();
        if !arg_value.ends_with('"') {
            return Err(ParserValidationError::NamedArgValueFormatError);
        }
        arg_value = arg_value.strip_suffix('"').unwrap().to_owned();
    } else {
        for c in arg_value.chars() {
            if !c.is_alphanumeric() {
                return Err(ParserValidationError::NamedArgFormatError);
            }
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

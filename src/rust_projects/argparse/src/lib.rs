pub mod parser_validations;

use core::fmt;
use std::{collections::hash_map, error::Error, vec};

#[derive(Debug)]
pub enum ParserError {
    FlagValidationError(parser_validations::ParserValidationError),
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

#[derive(Debug)]
pub enum ArgType {
    INTEGER,
    FLOAT,
    CHAR,
    STRING,
}

#[derive(Debug)]
#[allow(unused)]
struct FlagConfig {
    name: String,
    short_flag: char,
    long_flag: Option<String>,
    description: String,
}

#[derive(Debug)]
#[allow(unused)]
struct NamedArgumentConfig {
    name: String,
    argument: String,
    description: String,
    arg_type: ArgType,
}

#[derive(Debug)]
struct PositionalArgumentConfig {
    name: String,
    description: String,
    arg_type: ArgType,
}

#[derive(Debug)]
struct ArgConfigs {
    flags: Vec<FlagConfig>,
    optional_named_arguments: Vec<NamedArgumentConfig>,
    required_named_arguments: Vec<NamedArgumentConfig>,
    optional_positional_arguments: Vec<PositionalArgumentConfig>,
    required_positional_arguments: Vec<PositionalArgumentConfig>,
}

impl ArgConfigs {}

#[derive(Debug)]
pub enum ArgValue {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
}

#[allow(unused)]
#[derive(Debug)]
enum ArgToken {
    Flag(char),
    NamedArgument { arg: String, value: String },
    PositionalArgument(String),
}

pub struct Parser {
    arg_config: ArgConfigs,
    description: String,
    parsed_args: hash_map::HashMap<String, ArgValue>,
}

pub fn new_parser(description: String) -> Parser {
    Parser {
        arg_config: ArgConfigs {
            flags: vec![],
            optional_named_arguments: vec![],
            required_named_arguments: vec![],
            optional_positional_arguments: vec![],
            required_positional_arguments: vec![],
        },
        description: description,
        parsed_args: hash_map::HashMap::new(),
    }
}

impl Parser {
    pub fn add_flag(
        mut self,
        name: String,
        short_flag: char,
        long_flag: Option<String>,
        description: String,
    ) -> Self {
        self.arg_config.flags.push(FlagConfig {
            name: name,
            short_flag: short_flag,
            long_flag: long_flag,
            description: description,
        });

        self
    }

    pub fn add_named_argument(
        mut self,
        name: String,
        argument: String,
        required: bool,
        description: String,
        arg_type: ArgType,
    ) -> Self {
        if required {
            self.arg_config
                .required_named_arguments
                .push(NamedArgumentConfig {
                    name: name,
                    argument: argument,
                    description: description,
                    arg_type: arg_type,
                });
        } else {
            self.arg_config
                .optional_named_arguments
                .push(NamedArgumentConfig {
                    name: name,
                    argument: argument,
                    description: description,
                    arg_type: arg_type,
                });
        }

        self
    }

    /// Positional arguments parsed in order received
    pub fn add_positional_argument(
        mut self,
        name: String,
        required: bool,
        description: String,
        arg_type: ArgType,
    ) -> Self {
        if required {
            self.arg_config
                .required_positional_arguments
                .push(PositionalArgumentConfig {
                    name: name,
                    description: description,
                    arg_type: arg_type,
                });
        } else {
            self.arg_config
                .optional_positional_arguments
                .push(PositionalArgumentConfig {
                    name: name,
                    description: description,
                    arg_type: arg_type,
                });
        }

        self
    }

    pub fn get_bool_arg(&self, name: &str) -> Option<bool> {
        match self.parsed_args.get(name) {
            Some(ArgValue::Boolean(val)) => Some(*val),
            _ => None,
        }
    }

    pub fn get_integer_arg(&self, name: &str) -> Option<i32> {
        match self.parsed_args.get(name) {
            Some(ArgValue::Integer(val)) => Some(*val),
            _ => None,
        }
    }

    pub fn get_float_arg(&self, name: &str) -> Option<f32> {
        match self.parsed_args.get(name) {
            Some(ArgValue::Float(val)) => Some(*val),
            _ => None,
        }
    }

    pub fn get_string_arg(&self, name: &str) -> Option<&str> {
        match self.parsed_args.get(name) {
            Some(ArgValue::String(val)) => Some(val),
            _ => None,
        }
    }

    /// Parse the arguments
    fn do_parse(self, args: std::env::Args) -> Result<Self, ParserError> {
        // TODO: Handle -- to indicate end of options

        let mut toks: Vec<ArgToken> = Vec::new();

        // Parse the raw tokens (i.e. just as strings)
        for arg in args {
            if arg.starts_with("--") {
                // Process a named arg
                let arg_val_pair = arg.strip_prefix("--").unwrap().split_once('=').unwrap();
                // TODO: validations if '=' missing or invalid characters or length
                toks.push(ArgToken::NamedArgument {
                    arg: arg_val_pair.0.to_owned(),
                    value: arg_val_pair.1.to_owned(),
                });
            } else if arg.starts_with("-") {
                // Process a flag
                match parser_validations::validate_flag_format(&arg) {
                    Ok(()) => {}
                    Err(e) => {
                        return Err(ParserError::FlagValidationError(e));
                    }
                }
                let arg_values = arg.strip_prefix("-").unwrap().to_owned();
                for f in arg_values.chars().into_iter() {
                    toks.push(ArgToken::Flag(f));
                }
            } else {
                // Process positional arguments
                // TODO: validations invalid characters or length
                toks.push(ArgToken::PositionalArgument(arg.to_owned()));
            }
        }

        Ok(self)
    }

    pub fn parse(self) -> Result<Self, ParserError> {
        self.do_parse(std::env::args())
    }

    pub fn show_help(&self) {
        let mut help_output = format!("\n{}\n\n", &self.description);

        help_output.push_str("usage: COMMAND ");
        // Iterate through the config building the ouput for usage
        {
            // Flags
            if self.arg_config.flags.len() > 0 {
                help_output.push_str("[-");
            }
            for flag in &self.arg_config.flags {
                help_output.push_str(&format!("{}", flag.short_flag));
            }
            if self.arg_config.flags.len() > 0 {
                help_output.push_str("] ");
            }

            // Required named arguments
            for flag in &self.arg_config.required_named_arguments {
                help_output.push_str(&format!("--{}=VALUE ", flag.argument));
            }

            // Optional named arguments
            for flag in &self.arg_config.optional_named_arguments {
                help_output.push_str(&format!("[--{}=VALUE] ", flag.argument));
            }

            // Required positional arguments
            for flag in &self.arg_config.required_positional_arguments {
                help_output.push_str(&format!("{} ", flag.name));
            }

            // Required optional arguments
            for flag in &self.arg_config.optional_positional_arguments {
                help_output.push_str(&format!("[{}] ", flag.name));
            }
        } // end building usage line

        help_output.push_str("\n\n\n\tFlags:\n");

        for flag in &self.arg_config.flags {
            help_output.push_str(&format!("\t\t-{}\n", flag.short_flag));
            if let Some(long_flag) = &flag.long_flag {
                help_output.push_str(&format!("\t\t--{}\n", long_flag));
            }
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        help_output.push_str("\n\tNamed Arguments:\n");

        for flag in &self.arg_config.required_named_arguments {
            help_output.push_str(&format!(
                "\t\t--{}=VALUE\t{:?} (REQUIRED)\n",
                flag.argument, flag.arg_type
            ));
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        for flag in &self.arg_config.optional_named_arguments {
            help_output.push_str(&format!(
                "\t\t--{}=VALUE\t{:?}\n",
                flag.argument, flag.arg_type
            ));
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        help_output.push_str("\n\tPositional Arguments:\n");

        for flag in &self.arg_config.required_positional_arguments {
            help_output.push_str(&format!(
                "\t\t{}\t{:?} (REQUIRED)\n",
                flag.name, flag.arg_type
            ));
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        for flag in &self.arg_config.optional_positional_arguments {
            help_output.push_str(&format!("\t\t{}\t{:?}\n", flag.name, flag.arg_type));
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        help_output.push_str("\n\n");

        print!("{}", help_output);
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn add_flag() {
        let parser = crate::new_parser("My parser".to_string()).add_flag(
            "myflag".to_string(),
            'c',
            None,
            "Count chars".to_string(),
        );

        assert_eq!(parser.arg_config.flags.len(), 1);
    }
}

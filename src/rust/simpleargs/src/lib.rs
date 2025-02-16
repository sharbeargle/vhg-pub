use std::{collections::HashMap, vec};

mod arg_validators;

/// Specify what type the argument value should be
#[derive(PartialEq, Eq, Debug)]
pub enum ArgType {
    Character,
    Float,
    Integer,
    String,
}

/// A parsed argument
pub enum Arg {
    Character(char),
    Float(f32),
    Integer(i32),
    String(String),
    Boolean(bool),
}

pub struct FlagConfig {
    name: String,
    short_flag: Option<char>,
    long_flag: Option<String>,
    required: bool,
    // None implies boolean flag
    arg_type: Option<ArgType>,
    description: String,
}

pub struct ArgConfig {
    name: String,
    // Currently will be string no matter what is set
    // TODO: Fix this.
    arg_type: ArgType,
    required: bool,
    description: String,
}

// TODO: Define and implement how configuration will be stored
pub struct Parser {
    description: String,
    flag_configs: Vec<FlagConfig>,
    /// Map a flag to an index in flagConfigs
    flag_map: HashMap<String, usize>,
    arg_configs: Vec<ArgConfig>,
    /// name -> arg value
    parsed_args: HashMap<String, Arg>,
}

pub fn new(description: String) -> Parser {
    Parser {
        description: description,
        flag_configs: vec![],
        arg_configs: vec![],
        flag_map: HashMap::new(),
        parsed_args: HashMap::new(),
    }
}

impl Parser {
    pub fn add_flag(mut self, flag_config: FlagConfig) -> Self {
        if let Some(flag) = &flag_config.long_flag {
            self.flag_map.insert(flag.clone(), self.flag_configs.len());
        }
        if let Some(flag) = &flag_config.short_flag {
            self.flag_map
                .insert(flag.clone().to_string(), self.flag_configs.len());
        }
        self.flag_configs.push(flag_config);
        self
    }

    /// Add a positional argument to the configuration
    /// Parsed in order added. Adding required args after unrequired args will have undefined behavior.
    pub fn add_arg(mut self, arg_config: ArgConfig) -> Self {
        self.arg_configs.push(arg_config);
        self
    }

    /// Print the help screen
    pub fn print_help(&self) {}

    /// Parse the command line arguments
    /// Short flag: -f=<arg> | -f <arg> | -f<arg>
    /// Long flag: --flag=<arg> | --flag <arg>
    pub fn parse(mut self, input_args: impl Iterator<Item = String>) -> Self {
        let mut intermediate_args: Vec<String> = vec![];

        // Parse into intermediate format
        // TODO: Do some validations on the flag format
        for item in input_args {
            if item.starts_with("--") {
                match item.split_once('=') {
                    Some((flag, arg)) => {
                        if flag.len() < 2 || arg.len() < 1 {
                            panic!("Fix me: crashed because received flag w/ arg with no flag name or no arg");
                        }
                        intermediate_args.push(flag.to_owned());
                        intermediate_args.push(arg.to_owned());
                    }
                    None => {
                        intermediate_args.push(item);
                    }
                }
            } else if item.starts_with("-") {
                if item.len() < 3 {
                    intermediate_args.push(item);
                } else {
                    let (flag, arg) = item.split_at(2);
                    intermediate_args.push(flag.to_owned());
                    intermediate_args.push(arg.to_owned());
                }
            } else {
                intermediate_args.push(item);
            }
        }

        // TODO: parse from intermediate format
        for item in intermediate_args {}

        self
    }

    pub fn get_arg(&self, name: &str) -> Option<Arg> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

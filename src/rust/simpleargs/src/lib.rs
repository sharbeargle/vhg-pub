use std::{collections::HashMap, error::Error, vec};

mod arg_validators;
use arg_validators::*;

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
    None,
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
    command: String,
    flag_configs: Vec<FlagConfig>,
    /// Map a flag to an index in flagConfigs
    flag_map: HashMap<String, usize>,
    arg_configs: Vec<ArgConfig>,
    /// name -> arg value
    required_parsed_args: HashMap<String, Arg>,
    optional_parsed_args: HashMap<String, Arg>,
}

pub fn new(description: String) -> Parser {
    Parser {
        description: description,
        command: "".to_string(),
        flag_configs: vec![],
        arg_configs: vec![],
        flag_map: HashMap::new(),
        required_parsed_args: HashMap::new(),
        optional_parsed_args: HashMap::new(),
    }
}

#[derive(Debug)]
pub enum ParserError {
    FlagConfigError,
    ArgConfigError,
}

impl Parser {
    fn add_flag_config(&mut self, flag_config: FlagConfig) -> Result<(), ParserError> {
        // TODO: Validate flag names

        // Add mapping from long_flag to flag_config index
        if let Some(long_flag) = &flag_config.long_flag {
            let mut flag = "--".to_string();
            flag.push_str(long_flag);
            self.flag_map.insert(flag, self.flag_configs.len());
        }

        // Add mapping from short_flag to flag_config index
        if let Some(short_flag) = &flag_config.short_flag {
            let mut flag = "-".to_string();
            flag.push(*short_flag);
            self.flag_map
                .insert(flag, self.flag_configs.len());
        }

        // If neither long or short flags were specified, return error
        if let (None, None) = (&flag_config.long_flag, &flag_config.short_flag) {
            return Err(ParserError::FlagConfigError);
        }

        // If no arg_type specified, assume it's a boolean flag which defaults to false
        let flag_arg = match flag_config.arg_type {
            None => Arg::Boolean(false),
            _ => Arg::None,
        };

        if flag_config.required {
            self.required_parsed_args.insert(flag_config.name.clone(), flag_arg);
        } else {
            self.optional_parsed_args.insert(flag_config.name.clone(), flag_arg);
        }

        self.flag_configs.push(flag_config);
        
        Ok(())
    }

    fn add_arg_config(&mut self, arg_config: ArgConfig) -> Result<(), ParserError> {
        if arg_config.required {
            self.required_parsed_args.insert(arg_config.name.clone(), Arg::None);
        } else {
            self.optional_parsed_args.insert(arg_config.name.clone(), Arg::None);
        }

        Ok(())
    }
}

impl Parser {
    pub fn add_flag(mut self, flag_config: FlagConfig) -> Self {
        // TODO: Print useful debug information
        self.add_flag_config(flag_config).unwrap();
        self
    }

    /// Add a positional argument to the configuration
    /// Parsed in order added. Adding required args after unrequired args will have undefined behavior.
    pub fn add_arg(mut self, arg_config: ArgConfig) -> Self {
        // TODO: Print useful debug information
        self.add_arg_config(arg_config).unwrap();
        self
    }

    /// Print the help screen
    pub fn print_help(&self) {}

    /// Parse the command line arguments
    /// Short flag: -f=<arg> | -f <arg> | -f<arg>
    /// Long flag: --flag=<arg> | --flag <arg>
    pub fn parse(mut self, mut input_args: impl Iterator<Item = String>) -> Self {
        // Assume there is at least one arg and the first one is the command 
        if let Some(command) = input_args.next() {
            self.command = command;
        } else {
            panic!("input_args length is zero");
        }
        
        let mut intermediate_args: Vec<String> = vec![];
        // Parse into intermediate format
        // TODO: Do some validations on the flag format
        for item in input_args {
            if is_long_flag(&item) {
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
            } else if is_short_flag(&item) {
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
        for item in intermediate_args {
            if arg_validators::is_flag(&item) {
                if let Some(flag_config_idx) = self.flag_map.get(&item) {
                    let flag_config = &self.flag_configs[*flag_config_idx];
                    if flag_config.required {
                        match &flag_config.arg_type {
                            Some(arg_type) => {
                                /* 
                                TODO: Implement this
                                match arg_type {

                                } */
                            }
                            None =>{
                                let mut arg_value = self.required_parsed_args.get_mut(&flag_config.name).unwrap();
                                *arg_value = Arg::Boolean(true);
                            }
                        }
                    }
                } else {
                    panic!("Flag DNE");
                }
            }
        }

        self
    }

    pub fn get_arg(&self, name: &str) -> Option<&Arg> {
        if let Some(arg) = self.required_parsed_args.get(name) {
            return Some(arg);
        } else if let Some(arg) = self.optional_parsed_args.get(name) {
            return Some(arg);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

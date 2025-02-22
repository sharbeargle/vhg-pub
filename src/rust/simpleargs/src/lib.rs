use std::{collections::HashMap, rc::Rc, vec};

mod utils;
use utils::*;

#[derive(Debug)]
enum ParserError {
    FlagConfigError,
    ArgConfigError,
    MissingRequiredFlag,
    //TooManyPositionalArguments,
    ArgValueIsFlag,
    ArgValueCharIsString,
}

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

/// If no short_flag or long_flag then this is a positional argument
/// If no arg_type for short_flag or long_flag then this is boolean arg_type
/// TODO: Support variable length positional args natively
struct ArgConfig {
    name: String,
    short_flag: Option<char>,
    long_flag: Option<String>,
    required: bool,
    arg_type: Option<ArgType>,
    description: String,
}

pub struct Parser {
    description: String,
    command: String,
    flag_configs: Vec<Rc<ArgConfig>>,
    /// Map a flag to an index in flagConfigs
    flag_map: HashMap<String, Rc<ArgConfig>>,
    /// Vector of positional arg configs
    pos_arg_configs: Vec<ArgConfig>,
    /// name -> arg value
    parsed_args: HashMap<String, Arg>,
}

pub fn new(description: String) -> Parser {
    Parser {
        description: description,
        command: "".to_string(),
        flag_configs: vec![],
        pos_arg_configs: vec![],
        flag_map: HashMap::new(),
        parsed_args: HashMap::new(),
    }
}

impl Parser {
    /// Add a flag_config to the list of configs
    fn add_flag_config(&mut self, flag_config: ArgConfig) -> Result<(), ParserError> {
        if !utils::validate_flag_config(&flag_config) {
            return Err(ParserError::FlagConfigError);
        }

        let rc_flag_config: Rc<ArgConfig> = Rc::new(flag_config);

        // Add mapping from long_flag to flag_config
        if let Some(long_flag) = &rc_flag_config.long_flag {
            self.flag_map.insert(
                utils::add_dashes_to_long_flag(long_flag),
                rc_flag_config.clone(),
            );
        }

        // Add mapping from short_flag to flag_config
        if let Some(short_flag) = &rc_flag_config.short_flag {
            self.flag_map.insert(
                utils::add_dash_to_short_flag(*short_flag),
                rc_flag_config.clone(),
            );
        }

        self.flag_configs.push(rc_flag_config);
        Ok(())
    }

    fn add_arg_config(&mut self, arg_config: ArgConfig) -> Result<(), ParserError> {
        if !utils::validate_arg_config(&arg_config) {
            return Err(ParserError::ArgConfigError);
        }

        self.pos_arg_configs.push(arg_config);
        Ok(())
    }

    /// Takes iterator of Strings and splits up --flag=<val> into separate strings
    /// returning vector of strings or error.
    fn tokenize_flag_arg_values(
        &self,
        input_args: impl Iterator<Item = String>,
    ) -> Result<Vec<String>, ParserError> {
        let mut intermediate_args: Vec<String> = vec![];
        // Parse into intermediate format
        for item in input_args {
            if is_long_flag(&item) {
                match item.split_once('=') {
                    Some((flag, arg)) => {
                        // TODO: validate_flag()
                        intermediate_args.push(flag.to_owned());
                        intermediate_args.push(arg.to_owned());
                    }
                    None => intermediate_args.push(item),
                }
            } else if is_short_flag(&item) {
                if item.len() < 3 {
                    // 'Check if only flag e.g. -X'
                    intermediate_args.push(item);
                } else {
                    let (flag, arg) = item.split_at(2);
                    intermediate_args.push(flag.to_owned());
                    intermediate_args.push(arg.to_owned());
                }
            } else {
                // It's an argument
                intermediate_args.push(item);
            }
        }

        Ok(intermediate_args)
    }

    /// Parses a string representing the flag's argument value
    /// and return it as an Arg of type defined by arg_type.
    fn parse_flag_arg_value(&self, arg_type: &ArgType, arg: &str) -> Result<Arg, ParserError> {
        if utils::is_flag(&arg) {
            return Err(ParserError::ArgValueIsFlag);
        }

        match arg_type {
            ArgType::Character => {
                if arg.len() > 1 {
                    return Err(ParserError::ArgValueCharIsString);
                }

                Ok(Arg::Character(arg.chars().next().unwrap()))
            }
            ArgType::Float => Ok(Arg::Float(arg.parse().unwrap())),
            ArgType::Integer => Ok(Arg::Integer(arg.parse().unwrap())),
            ArgType::String => Ok(Arg::String(arg.to_owned())),
        }
    }

    /// Check whether all required flags have been set.
    /// Set all default flag values in parsed_args if unset.
    fn validate_flags(&mut self) -> Result<(), ParserError> {
        for item in &self.flag_configs {
            if self.parsed_args.contains_key(&item.name) {
                continue;
            }

            if item.required {
                return Err(ParserError::MissingRequiredFlag);
            }

            match item.arg_type {
                Some(_) => {
                    self.parsed_args.insert(item.name.clone(), Arg::None);
                }
                None => {
                    self.parsed_args
                        .insert(item.name.clone(), Arg::Boolean(false));
                }
            }
        }
        for item in &self.pos_arg_configs {
            if self.parsed_args.contains_key(&item.name) {
                continue;
            }

            if item.required {
                return Err(ParserError::MissingRequiredFlag);
            }
        }
        Ok(())
    }
}

impl Parser {
    pub fn add_flag(
        mut self,
        name: String,
        long_flag: Option<String>,
        short_flag: Option<char>,
        required: bool,
        arg_type: Option<ArgType>,
        description: String,
    ) -> Self {
        let flag_config = ArgConfig {
            name: name,
            short_flag: short_flag,
            long_flag: long_flag,
            required: required,
            arg_type: arg_type,
            description: description,
        };
        // TODO: Print useful debug information instead of panic!
        if let (None, None) = (&flag_config.long_flag, &flag_config.short_flag) {
            self.add_arg_config(flag_config).unwrap();
        } else {
            self.add_flag_config(flag_config).unwrap();
        }

        self
    }

    /// Print the help screen
    //TODO: Clean this up
    pub fn print_help(&self) {
        let mut help_output = format!("\n{}\n\n", &self.description);
        help_output.push_str("usage: COMMAND [options] ");

        for flag in &self.flag_configs {
            if !flag.required {
                continue;
            }

            if let Some(long_flag) = &flag.long_flag {
                help_output.push_str(&format!("--{}=<{}> ", long_flag, &flag.name));
            } else if let Some(short_flag) = &flag.short_flag {
                help_output.push_str(&format!("-{}=<{}> ", short_flag, &flag.name));
            }
        }

        for arg in &self.pos_arg_configs {
            if arg.required {
                help_output.push_str(&format!("<{}> ", &arg.name));
            } else {
                help_output.push_str(&format!("[<{}>] ", &arg.name));
            }
        }

        help_output.push_str("\n\nFlags:\n");

        for item in &self.flag_configs {
            if let (None, None) = (item.short_flag, &item.long_flag) {
                // TODO: Print positional output
            } else {
                if let Some(flag) = item.short_flag {
                    help_output.push_str(&format!("\n\t-{} ", flag));
                    if let Some(_arg_type) = &item.arg_type {
                        help_output.push_str(&format!("<{}> ", &item.name));
                    }
                }
                if let Some(flag) = &item.long_flag {
                    help_output.push_str(&format!("\n\t--{} ", flag));
                    if let Some(_arg_type) = &item.arg_type {
                        help_output.push_str(&format!("<{}> ", &item.name));
                    }
                }
                if item.required {
                    help_output.push_str("\n\t\t(required) ");
                }
                help_output.push_str(&format!("\n\t\t{}", &item.description));
                help_output.push('\n');
            }
        }

        help_output.push_str("\nPositional Arguments:\n");
        for item in &self.pos_arg_configs {
            help_output.push_str(&format!("\n\t{}", &item.name));
            if item.required {
                help_output.push_str("\n\t\t(required)");
            }
            help_output.push_str(&format!("\n\t\t{}", &item.description));
            help_output.push('\n');
        }

        println!("{}", help_output);
    }

    /// Parse the command line arguments
    /// Short flag: -f=<arg> | -f <arg> | -f<arg>
    /// Long flag: --flag=<arg> | --flag <arg>
    /// Panics.
    pub fn parse(mut self, mut input_args: impl Iterator<Item = String>) -> Self {
        // Assume there is at least one arg and the first one is the command
        self.command = input_args.next().unwrap();

        let mut tokenized_args = if let Ok(tokens) = self.tokenize_flag_arg_values(input_args) {
            tokens.into_iter()
        } else {
            panic!("Problem tokenizing args");
        };

        let mut pos_args_iter = self.pos_arg_configs.iter();

        while let Some(item) = tokenized_args.next() {
            if utils::is_flag(&item) {
                let flag_config = if let Some(flag_config) = self.flag_map.get(&item) {
                    flag_config
                } else {
                    panic!("Flag not defined");
                };

                let parsed_arg = match &flag_config.arg_type {
                    // None implies boolean
                    None => Arg::Boolean(true),
                    // Having an arg_type means we need to parse the next arg
                    Some(arg_type) => {
                        let next_arg = match tokenized_args.next() {
                            None => panic!("expected argument"),
                            Some(arg) => arg,
                        };

                        self.parse_flag_arg_value(arg_type, &next_arg).unwrap()
                    }
                };

                self.parsed_args
                    .insert(flag_config.name.clone(), parsed_arg);
            } else {
                // Parse positional argument
                if let Some(flag_config) = pos_args_iter.next() {
                    let arg_type = flag_config.arg_type.as_ref().unwrap();
                    self.parsed_args.insert(
                        flag_config.name.clone(),
                        self.parse_flag_arg_value(arg_type, &item).unwrap(),
                    );
                } else {
                    panic!("got more positional arguements than configured");
                }
            }
        }

        if let Err(e) = self.validate_flags() {
            println!("Error: {:?}", e);
            panic!();
        }

        self
    }

    /// Retrieve the parsed arg value
    pub fn get_arg(&self, name: &str) -> Option<&Arg> {
        self.parsed_args.get(name)
    }
}

//TODO: Left off here: Create tests to test every condition
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let args: Vec<String> = vec![
            "command".to_string(),
            "--verbose".to_string(),
            "--flag=flagvalue".to_string(),
            "--optionalFlag".to_string(),
            "5".to_string(),
            "posargvalue".to_string(),
        ];

        let p = new("test parser".to_string())
            .add_flag(
                "verbose".to_string(),
                Some("verbose".to_string()),
                Some('v'),
                false,
                None,
                "Flag verbose".to_string(),
            )
            .add_flag(
                "myflag".to_string(),
                Some("flag".to_string()),
                Some('f'),
                true,
                Some(ArgType::String),
                "Flag to test string".to_string(),
            )
            .add_flag(
                "optionalFlag".to_string(),
                Some("optionalFlag".to_string()),
                None,
                false,
                Some(ArgType::Integer),
                "Test optional flag".to_string(),
            )
            .add_flag(
                "posArg".to_string(),
                None,
                None,
                true,
                Some(ArgType::String),
                "Test positional argument".to_string(),
            )
            .parse(args.into_iter());

        p.print_help();

        assert!(p.get_arg("verbose").is_some());
        assert!(p.get_arg("myflag").is_some());

        let arg = p.get_arg("optionalFlag");
        assert!(matches!(arg, Some(&Arg::None)));
    }
}

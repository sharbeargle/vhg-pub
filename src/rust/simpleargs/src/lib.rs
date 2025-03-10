use std::{collections::HashMap, process::exit, rc::Rc, vec};

mod utils;
use utils::*;

#[derive(Debug)]
enum ParserError {
    FlagConfigError,
    ArgConfigError,
    MissingRequiredFlag,
    //TooManyPositionalArguments,
    ArgValueIsFlag,
    IncorrectArgType,
}

#[derive(PartialEq, Debug)]
enum FlagType {
    ShortFlag,
    LongFlag,
    PositionalArgument,
}

/// Specify what type the argument value should be
#[derive(PartialEq, Debug)]
pub enum ArgType {
    Character,
    Float,
    Integer,
    String,
}

/// A parsed argument
#[derive(PartialEq, Debug)]
pub enum Arg {
    Character(char),
    Float(f32),
    Integer(i32),
    String(String),
    Boolean(bool),
    None,
}

/// If no short_flag or long_flag then this is a positional argument.
/// If no arg_type for short_flag or long_flag then this is boolean arg_type.
// TODO: Support variable length positional args natively
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

/// Instantiate a new arg parser
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

    /// Add a positional arg config to the list of configs
    fn add_pos_arg_config(&mut self, arg_config: ArgConfig) -> Result<(), ParserError> {
        if !utils::validate_arg_config(&arg_config) {
            return Err(ParserError::ArgConfigError);
        }

        self.pos_arg_configs.push(arg_config);
        Ok(())
    }

    /// "--flag=value" becomes ["--flag", "value"]
    fn split_long_flag(&self, long_flag: &str) -> Vec<String> {
        match long_flag.split_once('=') {
            Some((flag, arg)) => {
                vec![flag.to_owned(), arg.to_owned()]
            }
            None => vec![long_flag.to_owned()],
        }
    }

    /// Whether it's "-f=value", "-fvalue", or "-f value", it becomes ["-f", "value"]
    fn split_short_flag(&self, long_flag: &str) -> Vec<String> {
        // short flag with length < 3 is just the single char
        if long_flag.len() < 3 {
            vec![long_flag.to_owned()]
        } else {
            let (flag, arg) = long_flag.split_at(2);
            let arg_val = if let Some(stripped_arg) = arg.strip_prefix('=') {
                stripped_arg.to_owned()
            } else {
                arg.to_owned()
            };

            vec![flag.to_owned(), arg_val]
        }
    }

    fn get_flag_type(&self, item: &str) -> FlagType {
        if is_long_flag(&item) {
            FlagType::LongFlag
        } else if is_short_flag(&item) {
            FlagType::ShortFlag
        } else {
            FlagType::PositionalArgument
        }
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
            let split_flag: Vec<String> = match self.get_flag_type(&item) {
                FlagType::LongFlag => self.split_long_flag(&item),
                FlagType::ShortFlag => self.split_short_flag(&item),
                FlagType::PositionalArgument => vec![item],
            };

            for val in split_flag {
                intermediate_args.push(val);
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
                if arg.len() != 1 {
                    return Err(ParserError::IncorrectArgType);
                }

                // Unwrap is okay here because of the length check above
                Ok(Arg::Character(arg.chars().next().unwrap()))
            }
            ArgType::Float => match arg.parse::<f32>() {
                Err(_) => Err(ParserError::IncorrectArgType),
                Ok(val) => Ok(Arg::Float(val)),
            },
            ArgType::Integer => match arg.parse::<i32>() {
                Err(_) => Err(ParserError::IncorrectArgType),
                Ok(val) => Ok(Arg::Integer(val)),
            },
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

            // For non-required fields, set their arg values
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

            self.parsed_args.insert(item.name.clone(), Arg::None);
        }
        Ok(())
    }
}

impl Parser {
    /// Define a flag configuration
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
        // Unwrap here is okay because this indicates a bug in the
        // configuration, not due to end user inputing incorrect args
        if let (None, None) = (&flag_config.long_flag, &flag_config.short_flag) {
            self.add_pos_arg_config(flag_config).unwrap();
        } else {
            self.add_flag_config(flag_config).unwrap();
        }

        self
    }

    /// Print the help screen
    //TODO: Print argument type correctly
    pub fn print_help(&self) {
        let mut help_output = format!("\n{}\n\n", &self.description);
        help_output.push_str("usage: COMMAND [options] ");

        for flag in &self.flag_configs {
            if !flag.required {
                continue;
            }

            if let Some(long_flag) = &flag.long_flag {
                help_output.push_str(&format!(
                    "--{}=<{}:{:?}> ",
                    long_flag, &flag.name, &flag.arg_type
                ));
            } else if let Some(short_flag) = &flag.short_flag {
                help_output.push_str(&format!(
                    "-{}=<{}:{:?}> ",
                    short_flag, &flag.name, &flag.arg_type
                ));
            }
        }

        for arg in &self.pos_arg_configs {
            if arg.required {
                help_output.push_str(&format!("<{}:{:?}> ", &arg.name, &arg.arg_type));
            } else {
                help_output.push_str(&format!("[<{}:{:?}>] ", &arg.name, &arg.arg_type));
            }
        }

        help_output.push_str("\n\nFlags:\n");

        help_output.push_str(&format!("\n\t--help "));
        help_output.push_str(&format!("\n\t\tPrint this help page\n"));

        for item in &self.flag_configs {
            if let Some(flag) = item.short_flag {
                help_output.push_str(&format!("\n\t-{} ", flag));
                if let Some(arg_type) = &item.arg_type {
                    help_output.push_str(&format!("<{}:{:?}> ", &item.name, arg_type));
                }
            }
            if let Some(flag) = &item.long_flag {
                help_output.push_str(&format!("\n\t--{} ", flag));
                if let Some(_arg_type) = &item.arg_type {
                    help_output.push_str(&format!("<{}:{:?}> ", &item.name, &item.arg_type));
                }
            }
            if item.required {
                help_output.push_str("\n\t\t(required) ");
            }
            help_output.push_str(&format!("\n\t\t{}", &item.description));
            help_output.push('\n');
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
    ///
    /// - Short flag: -f=<arg> | -f <arg> | -f<arg>
    /// - Long flag: --flag=<arg> | --flag <arg>
    // TODO: Simplify this function. It is too complicated.
    pub fn parse(mut self, mut input_args: impl Iterator<Item = String>) -> Self {
        // There must be at least one arg and the first one is the command
        self.command = if let Some(arg) = input_args.next() {
            arg
        } else {
            println!("Must have more than one argument (the command)");
            self.print_help();
            exit(-1);
        };

        // Tokenize all the flag argument values:
        // e.g. "--flag=value" turns into ["--flag", "value"]
        let mut tokenized_args = if let Ok(tokens) = self.tokenize_flag_arg_values(input_args) {
            tokens.into_iter()
        } else {
            println!("Problem tokenizing args");
            self.print_help();
            exit(-1);
        };

        // Since positional args don't have flags, they can't be mapped to
        // specific configs. To track which args map to which config, we
        // consume the configs as an iteratator as we parse.
        let mut pos_args_iter = self.pos_arg_configs.iter();

        // Main parsing loop
        while let Some(item) = tokenized_args.next() {
            if &item == "--help" {
                self.print_help();
                exit(0);
            }

            if utils::is_flag(&item) {
                let flag_config = if let Some(flag_config) = self.flag_map.get(&item) {
                    flag_config
                } else {
                    println!("Flag not defined");
                    self.print_help();
                    exit(-1);
                };

                let parsed_arg = match &flag_config.arg_type {
                    // None implies boolean
                    None => Arg::Boolean(true),
                    // Having an arg_type means we need to parse the next arg
                    Some(arg_type) => {
                        let next_arg = match tokenized_args.next() {
                            None => {
                                println!("expected argument");
                                self.print_help();
                                exit(-1);
                            }
                            Some(arg) => arg,
                        };

                        match self.parse_flag_arg_value(arg_type, &next_arg) {
                            Ok(val) => val,
                            Err(e) => {
                                println!("{:?}", e);
                                self.print_help();
                                exit(-1);
                            }
                        }
                    }
                };

                self.parsed_args
                    .insert(flag_config.name.clone(), parsed_arg);
            } else {
                // Parse positional argument
                if let Some(flag_config) = pos_args_iter.next() {
                    // Unwrap is safe here because we validate upon insertion
                    // that positional args have an arg_type != None
                    let arg_type = flag_config.arg_type.as_ref().unwrap();
                    let arg_value = match self.parse_flag_arg_value(arg_type, &item) {
                        Ok(val) => val,
                        Err(e) => {
                            println!("{:?}", e);
                            self.print_help();
                            exit(-1);
                        }
                    };
                    self.parsed_args.insert(flag_config.name.clone(), arg_value);
                } else {
                    println!("got more positional arguements than configured");
                    self.print_help();
                    exit(-1);
                }
            }
        }

        if let Err(e) = self.validate_flags() {
            println!("Error: {:?}", e);
            self.print_help();
            exit(-1);
        }

        self
    }

    /// Retrieve the parsed arg value
    pub fn get_arg(&self, name: &str) -> Option<&Arg> {
        self.parsed_args.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test missing required flags are returning errors appropriately
    // TODO: Test incorrect arg type are returning errors appropriately

    // Test missing non-required flags are set appropriately
    #[test]
    fn test_non_required_flags() {
        let input_args: Vec<String> = vec!["command".to_string()];

        let p = new("test parser".to_string())
            .add_flag(
                "integer".to_string(),
                Some("integer".to_string()),
                None,
                false,
                Some(ArgType::Integer),
                "Test a integer".to_string(),
            )
            .add_flag(
                "float".to_string(),
                Some("float".to_string()),
                None,
                false,
                Some(ArgType::Float),
                "Test an Float".to_string(),
            )
            .add_flag(
                "character".to_string(),
                Some("character".to_string()),
                None,
                false,
                Some(ArgType::Character),
                "Test an Character".to_string(),
            )
            .add_flag(
                "string".to_string(),
                Some("string".to_string()),
                None,
                false,
                Some(ArgType::String),
                "Test an String".to_string(),
            )
            .add_flag(
                "boolean".to_string(),
                Some("boolean".to_string()),
                None,
                false,
                None,
                "Test an boolean".to_string(),
            )
            .add_flag(
                "positional_integer".to_string(),
                None,
                None,
                false,
                Some(ArgType::Integer),
                "Test an Integer".to_string(),
            )
            .parse(input_args.into_iter());

        let arg_res = p.get_arg("integer");
        assert!(matches!(arg_res, Some(Arg::None)));

        let arg_res = p.get_arg("float");
        assert!(matches!(arg_res, Some(Arg::None)));

        let arg_res = p.get_arg("character");
        assert!(matches!(arg_res, Some(Arg::None)));

        let arg_res = p.get_arg("string");
        assert!(matches!(arg_res, Some(Arg::None)));

        let arg_res = p.get_arg("boolean");
        assert!(matches!(arg_res, Some(Arg::Boolean(false))));

        let arg_res = p.get_arg("positional_integer");
        assert!(matches!(arg_res, Some(Arg::None)));
    }

    /// Verify args of various types are parsed correctly
    #[test]
    fn test_various_arg_types() {
        let input_args: Vec<String> = vec![
            "command".to_string(),
            "--integer=50".to_string(),
            "--float=0.401".to_string(),
            "--character=x".to_string(),
            "--string=string_of_characters".to_string(),
            "--boolean".to_string(),
            "100".to_string(),
            "4.34".to_string(),
            "y".to_string(),
            "string_of_stuff".to_string(),
        ];

        let p = new("test parser".to_string())
            .add_flag(
                "integer".to_string(),
                Some("integer".to_string()),
                None,
                true,
                Some(ArgType::Integer),
                "Test an Integer".to_string(),
            )
            .add_flag(
                "float".to_string(),
                Some("float".to_string()),
                None,
                true,
                Some(ArgType::Float),
                "Test an Float".to_string(),
            )
            .add_flag(
                "character".to_string(),
                Some("character".to_string()),
                None,
                true,
                Some(ArgType::Character),
                "Test an Character".to_string(),
            )
            .add_flag(
                "string".to_string(),
                Some("string".to_string()),
                None,
                true,
                Some(ArgType::String),
                "Test an String".to_string(),
            )
            .add_flag(
                "boolean".to_string(),
                Some("boolean".to_string()),
                None,
                true,
                None,
                "Test an boolean".to_string(),
            )
            .add_flag(
                "positional_integer".to_string(),
                None,
                None,
                true,
                Some(ArgType::Integer),
                "Test an Integer".to_string(),
            )
            .add_flag(
                "positional_float".to_string(),
                None,
                None,
                true,
                Some(ArgType::Float),
                "Test an Float".to_string(),
            )
            .add_flag(
                "positional_character".to_string(),
                None,
                None,
                true,
                Some(ArgType::Character),
                "Test an Character".to_string(),
            )
            .add_flag(
                "positional_string".to_string(),
                None,
                None,
                true,
                Some(ArgType::String),
                "Test an String".to_string(),
            )
            .parse(input_args.into_iter());

        let arg_res = p.get_arg("integer");
        assert!(matches!(arg_res, Some(Arg::Integer(50))));

        let arg_res = p.get_arg("float");
        assert!(matches!(arg_res, Some(Arg::Float(0.401))));

        let arg_res = p.get_arg("character");
        assert!(matches!(arg_res, Some(Arg::Character('x'))));

        let arg_res = p.get_arg("string");
        assert!(matches!(arg_res, Some(Arg::String(_))));

        let arg_res = p.get_arg("boolean");
        assert!(matches!(arg_res, Some(Arg::Boolean(true))));

        let arg_res = p.get_arg("positional_integer");
        assert!(matches!(arg_res, Some(Arg::Integer(100))));

        let arg_res = p.get_arg("positional_float");
        assert!(matches!(arg_res, Some(Arg::Float(4.34))));

        let arg_res = p.get_arg("positional_character");
        assert!(matches!(arg_res, Some(Arg::Character('y'))));

        let arg_res = p.get_arg("positional_string");
        assert!(matches!(arg_res, Some(Arg::String(_))));
    }

    /// Verify we can pass boolean short and long flags correctly
    #[test]
    fn test_bool_flags() {
        let input_args: Vec<String> = vec![
            "command".to_string(),
            "-f".to_string(),
            "--g_flag".to_string(),
        ];

        let p = new("test parser".to_string())
            .add_flag(
                "f_flag".to_string(),
                None,
                Some('f'),
                true,
                None,
                "f_flag tests -f=f_arg_val".to_string(),
            )
            .add_flag(
                "g_flag".to_string(),
                Some("g_flag".to_string()),
                None,
                true,
                None,
                "g_flag tests -gg_arg_val".to_string(),
            )
            .parse(input_args.into_iter());

        let f = p.get_arg("f_flag");
        assert!(matches!(f, Some(Arg::Boolean(true))));

        let g = p.get_arg("g_flag");
        assert!(matches!(g, Some(Arg::Boolean(true))));
    }

    /// Verify that we can parse long flags with arguments correctly
    /// regardless of the various syntax options for passing arguments
    #[test]
    fn test_long_flags_with_args() {
        let input_args: Vec<String> = vec![
            "command".to_string(),
            "--f_flag=f_arg_val".to_string(),
            "--g_flag".to_string(),
            "g_arg_val".to_string(),
        ];

        let p = new("test parser".to_string())
            .add_flag(
                "f_flag".to_string(),
                Some("f_flag".to_string()),
                None,
                true,
                Some(ArgType::String),
                "f_flag tests -f=f_arg_val".to_string(),
            )
            .add_flag(
                "g_flag".to_string(),
                Some("g_flag".to_string()),
                None,
                true,
                Some(ArgType::String),
                "g_flag tests -gg_arg_val".to_string(),
            )
            .parse(input_args.into_iter());

        let f_val = "f_arg_val".to_string();
        let f = p.get_arg("f_flag");
        assert!(matches!(f, Some(Arg::String(_))));
        let f = f.unwrap();
        match f {
            Arg::String(s) => {
                assert_eq!(s, &f_val);
            }
            _ => {}
        }

        let g_val = "g_arg_val".to_string();
        let g = p.get_arg("g_flag");
        assert!(matches!(g, Some(Arg::String(_))));
        let g = g.unwrap();
        match g {
            Arg::String(s) => {
                assert_eq!(s, &g_val);
            }
            _ => {}
        }
    }

    #[test]
    /// Verify that we can parse short flags with arguments correctly
    /// regardless of the various syntax options for passing arguments
    fn test_short_flags_with_args() {
        let input_args: Vec<String> = vec![
            "command".to_string(),
            "-f=f_arg_val".to_string(),
            "-gg_arg_val".to_string(),
            "-h".to_string(),
            "h_arg_val".to_string(),
        ];

        let p = new("test parser".to_string())
            .add_flag(
                "f_flag".to_string(),
                None,
                Some('f'),
                true,
                Some(ArgType::String),
                "f_flag tests -f=f_arg_val".to_string(),
            )
            .add_flag(
                "g_flag".to_string(),
                None,
                Some('g'),
                true,
                Some(ArgType::String),
                "g_flag tests -gg_arg_val".to_string(),
            )
            .add_flag(
                "h_flag".to_string(),
                None,
                Some('h'),
                true,
                Some(ArgType::String),
                "h_flag tests -h h_arg_val".to_string(),
            )
            .parse(input_args.into_iter());

        let f_val = "f_arg_val".to_string();
        let f = p.get_arg("f_flag");
        assert!(matches!(f, Some(Arg::String(_))));
        let f = f.unwrap();
        match f {
            Arg::String(s) => {
                assert_eq!(s, &f_val);
            }
            _ => {}
        }

        let g_val = "g_arg_val".to_string();
        let g = p.get_arg("g_flag");
        assert!(matches!(g, Some(Arg::String(_))));
        let g = g.unwrap();
        match g {
            Arg::String(s) => {
                assert_eq!(s, &g_val);
            }
            _ => {}
        }

        let h_val = "h_arg_val".to_string();
        let h = p.get_arg("h_flag");
        assert!(matches!(h, Some(Arg::String(_))));
        let h = h.unwrap();
        match h {
            Arg::String(s) => {
                assert_eq!(s, &h_val);
            }
            _ => {}
        }
    }

    #[test]
    fn it_works() {
        let args: Vec<String> = vec![
            "command".to_string(),
            "--verbose".to_string(),
            "--flag=flagvalue".to_string(),
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

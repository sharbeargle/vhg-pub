use std::vec;

#[derive(Debug)]
pub enum ArgType {
    INTEGER,
    FLOAT,
    CHAR,
    STRING,
}

#[derive(Debug)]
struct FlagConfig {
    flag: char,
    description: String,
}

#[derive(Debug)]
struct NamedArgumentConfig {
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
pub struct ArgConfigs {
    optional_flags: Vec<FlagConfig>,
    required_flags: Vec<FlagConfig>,
    optional_named_arguments: Vec<NamedArgumentConfig>,
    required_named_arguments: Vec<NamedArgumentConfig>,
    optional_positional_arguments: Vec<PositionalArgumentConfig>,
    required_positional_arguments: Vec<PositionalArgumentConfig>,
}

pub fn new_argconfig() -> ArgConfigs {
    ArgConfigs {
        optional_flags: vec![],
        required_flags: vec![],
        optional_named_arguments: vec![],
        required_named_arguments: vec![],
        optional_positional_arguments: vec![],
        required_positional_arguments: vec![],
    }
}

impl ArgConfigs {
    pub fn add_flag(mut self, flag: char, required: bool, description: String) -> Self {
        if required {
            self.required_flags.push(FlagConfig {
                flag: flag,
                description: description,
            });
        } else {
            self.optional_flags.push(FlagConfig {
                flag: flag,
                description: description,
            });
        }

        self
    }

    pub fn add_named_argument(
        mut self,
        argument: String,
        required: bool,
        description: String,
        arg_type: ArgType,
    ) -> Self {
        if required {
            self.required_named_arguments.push(NamedArgumentConfig {
                argument: argument,
                description: description,
                arg_type: arg_type,
            });
        } else {
            self.optional_named_arguments.push(NamedArgumentConfig {
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
            self.required_positional_arguments
                .push(PositionalArgumentConfig {
                    name: name,
                    description: description,
                    arg_type: arg_type,
                });
        } else {
            self.optional_positional_arguments
                .push(PositionalArgumentConfig {
                    name: name,
                    description: description,
                    arg_type: arg_type,
                });
        }

        self
    }
}

pub struct Parser {
    arg_config: ArgConfigs,
    description: String,
}

pub fn new_parser(arg_config: ArgConfigs, description: String) -> Parser {
    Parser {
        arg_config: arg_config,
        description: description,
    }
}

impl Parser {
    pub fn show_help(&self) {
        let mut help_output = format!("\n{}\n\n", &self.description);

        help_output.push_str("usage: COMMAND ");
        // Iterate through the config building the ouput for usage
        {
            // Required flags
            for flag in &self.arg_config.required_flags {
                help_output.push_str(&format!("-{} ", flag.flag));
            }

            // Optional flags
            if self.arg_config.optional_flags.len() > 0 {
                help_output.push_str("[-");
            }
            for flag in &self.arg_config.optional_flags {
                help_output.push_str(&format!("{}", flag.flag));
            }
            if self.arg_config.optional_flags.len() > 0 {
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

        for flag in &self.arg_config.required_flags {
            help_output.push_str(&format!("\t\t-{}\t(REQUIRED)\n", flag.flag));
            help_output.push_str(&format!("\t\t\t{}\n\n", flag.description));
        }

        for flag in &self.arg_config.optional_flags {
            help_output.push_str(&format!("\t\t-{}\n", flag.flag));
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
        let arg_config = crate::new_argconfig().add_flag('c', false, "Count chars".to_string());

        assert_eq!(arg_config.optional_flags.len(), 1);
        assert_eq!(arg_config.required_flags.len(), 0);
    }
}

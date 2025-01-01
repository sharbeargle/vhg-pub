use std::vec;

#[derive(Debug)]
struct ShortFlagConfig {
    flag: char,
    required: bool,
    description: String,
}

#[derive(Debug)]
struct LongFlagConfig {
    argument: String,
    required: bool,
    with_value: bool,
    description: String,
}

#[derive(Debug)]
struct ShortLongFlagConfig {
    flag: char,
    argument: String,
    required: bool,
    with_value: bool,
    description: String,
}

#[derive(Debug)]
struct PositionalArgumentConfig {
    name: String,
    required: bool,
    description: String,
}

#[derive(Debug)]
pub struct ArgConfigs {
    short_flags: Vec<ShortFlagConfig>,
    long_flags: Vec<LongFlagConfig>,
    short_long_flags: Vec<ShortLongFlagConfig>,
    positionals: Vec<PositionalArgumentConfig>,
}

pub fn new_argconfig() -> ArgConfigs {
    ArgConfigs {
        short_flags: vec![],
        long_flags: vec![],
        short_long_flags: vec![],
        positionals: vec![],
    }
}

impl ArgConfigs {
    pub fn add_flag(mut self, flag: char, required: bool, description: String) -> Self {
        self.short_flags.push(ShortFlagConfig {
            flag: flag,
            required: required,
            description: description,
        });
        self
    }

    pub fn add_argument(
        mut self,
        argument: String,
        required: bool,
        with_value: bool,
        description: String,
    ) -> Self {
        self.long_flags.push(LongFlagConfig {
            argument: argument,
            required: required,
            with_value: with_value,
            description: description,
        });
        self
    }

    pub fn add_flag_with_argument(
        mut self,
        flag: char,
        argument: String,
        required: bool,
        with_value: bool,
        description: String,
    ) -> Self {
        self.short_long_flags.push(ShortLongFlagConfig {
            flag: flag,
            argument: argument,
            required: required,
            with_value: with_value,
            description: description,
        });
        self
    }

    /// Positional arguments parsed in order received
    pub fn add_positional_argument(
        mut self,
        name: String,
        required: bool,
        description: String,
    ) -> Self {
        self.positionals.push(PositionalArgumentConfig {
            name: name,
            required: required,
            description: description,
        });
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
        println!();
        println!();
        println!("{}", &self.description);
        println!();
        println!();
        
        // Iterate through the config building the ouput
        print!("COMMAND ");

        for flag in &self.arg_config.short_flags {
            if flag.required {
                print!("-{} ", flag.flag);
            }
        }

        for flag in &self.arg_config.long_flags {
            if flag.required {
                print!("--{} ", flag.argument);
            }
            if flag.with_value {
                print!("VALUE ");
            }
        }

        for flag in &self.arg_config.short_long_flags {
            if flag.required {
                print!("--{} ", flag.argument);
            }
            if flag.with_value {
                print!("VALUE ");
            }
        }

        for flag in &self.arg_config.positionals {
            if flag.required {
                print!("{} ", flag.name);
            }
        }

        println!();
        println!();

        for flag in &self.arg_config.short_flags {
            let required = match flag.required {
                true => " REQUIRED ",
                false => " ",
            };
            println!("    -{}{}{}", flag.flag, required, flag.description);
        }
        for flag in &self.arg_config.long_flags {
            let required = match flag.required {
                true => " REQUIRED ",
                false => " ",
            };
            let has_value = match flag.with_value {
                true => "=VALUE ",
                false => " ",
            };
            println!(
                "    --{}{}{}{}",
                flag.argument, has_value, required, flag.description
            );
        }
        for flag in &self.arg_config.short_long_flags {
            let required = match flag.required {
                true => " REQUIRED ",
                false => " ",
            };
            let has_value = match flag.with_value {
                true => "=VALUE ",
                false => " ",
            };
            println!(
                "    -{}, --{}{}{}{}",
                flag.flag, flag.argument, has_value, required, flag.description
            );
        }

        if self.arg_config.positionals.len() > 0 {
            println!();
            println!("Positionals");
        }
        for flag in &self.arg_config.positionals {
            let required = match flag.required {
                true => " REQUIRED ",
                false => " ",
            };

            println!("    {} {}: {}", flag.name, required, flag.description);
        }
        println!();
        println!();
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let arg_config = crate::new_argconfig().add_flag('c', false, "Count chars".to_string());

        assert_eq!(arg_config.short_flags.len(), 1);
    }
}

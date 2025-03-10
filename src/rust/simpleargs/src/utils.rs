use super::ArgConfig;

pub fn add_dashes_to_long_flag(long_flag: &str) -> String {
    let mut flag = "--".to_string();
    flag.push_str(long_flag);
    flag
}

pub fn add_dash_to_short_flag(short_flag: char) -> String {
    let mut flag = "-".to_string();
    flag.push(short_flag);
    flag
}

pub fn validate_flag_config(_arg_config: &ArgConfig) -> bool {
    // If neither long or short flags were specified, return error
    /*
    if let (None, None) = (&flag_config.long_flag, &flag_config.short_flag) {
        return Err(ParserError::FlagConfigError);
    }
     */

    // Validate flag allowed characters

    true
}

pub fn validate_arg_config(_arg_config: &ArgConfig) -> bool {
    true
}

/*
pub fn validate_flag_arg() -> bool {

    if flag.len() < 2 || arg.len() < 1 {
                            panic!("Fix me: crashed because received flag w/ arg with no flag name or no arg");
                        }


    true
}

     */

pub fn is_flag(arg: &str) -> bool {
    arg.starts_with('-')
}

pub fn is_short_flag(arg: &str) -> bool {
    arg.starts_with('-')
}

pub fn is_long_flag(arg: &str) -> bool {
    arg.starts_with("--")
}

pub fn validate_flag_config() -> bool {
    // If neither long or short flags were specified, return error
    /*
    if let (None, None) = (&flag_config.long_flag, &flag_config.short_flag) {
        return Err(ParserError::FlagConfigError);
    }
     */

    // Validate flag allowed characters

    true
}

pub fn validate_flag_arg() -> bool {
    /*
    if flag.len() < 2 || arg.len() < 1 {
                            panic!("Fix me: crashed because received flag w/ arg with no flag name or no arg");
                        }
                         */

    true
}

pub fn is_flag(arg: &str) -> bool {
    arg.starts_with('-')
}

pub fn is_short_flag(arg: &str) -> bool {
    arg.starts_with('-')
}

pub fn is_long_flag(arg: &str) -> bool {
    arg.starts_with("--")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

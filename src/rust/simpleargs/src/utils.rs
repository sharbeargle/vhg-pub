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

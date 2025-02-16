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

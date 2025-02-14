use argparse;

fn main() {
    let parser = argparse::new_parser(
        "This is an example app showing how to use argparse to parse commands".to_string(),
    )
    .add_flag(
        "myflaga".to_string(),
        'a',
        None,
        "Enable flag a".to_string(),
    )
    .add_flag(
        "myflagb".to_string(),
        'B',
        Some("Bflag".to_string()),
        "Enable flag B".to_string(),
    )
    .add_named_argument(
        "myarg1".to_string(),
        "arg1".to_string(),
        true,
        "Arg1 input".to_string(),
        argparse::ArgType::INTEGER,
    )
    .add_named_argument(
        "myarg2".to_string(),
        "arg2".to_string(),
        false,
        "arg2 input".to_string(),
        argparse::ArgType::STRING,
    )
    .add_named_argument(
        "myarg3".to_string(),
        "arg3".to_string(),
        false,
        "arg3 input".to_string(),
        argparse::ArgType::CHAR,
    )
    .add_positional_argument(
        "POS1".to_string(),
        true,
        "Positional argument 1".to_string(),
        argparse::ArgType::INTEGER,
    )
    .add_positional_argument(
        "POS2".to_string(),
        false,
        "Positional argument 2".to_string(),
        argparse::ArgType::STRING,
    )
    .parse()
    .unwrap();

    parser.show_help();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}

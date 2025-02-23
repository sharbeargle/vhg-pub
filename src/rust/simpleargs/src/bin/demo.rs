use simpleargs::{ArgType, Parser};

fn parse_args() -> Parser {
    simpleargs::new("test parser".to_string())
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
        .parse(std::env::args())
}

fn main() {
    let p = parse_args();

    println!("Demo of simpleargs usage");
    println!("verbose: {:?}", p.get_arg("verbose"));
    println!("myflag: {:?}", p.get_arg("myflag"));
    println!("optionalFlag: {:?}", p.get_arg("optionalFlag"));
    println!("posArg: {:?}", p.get_arg("posArg"));
}

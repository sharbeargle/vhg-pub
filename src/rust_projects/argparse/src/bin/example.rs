use argparse;

fn main() {
    let arg_config = argparse::new_argconfig()
        .add_flag('a', true, "Enable flag a".to_string())
        .add_flag('B', true, "Enable flag B".to_string())
        .add_flag('c', false, "Enable flag c".to_string())
        .add_flag('D', false, "Enable flag D".to_string())
        .add_named_argument(
            "arg1".to_string(),
            true,
            "Arg1 input".to_string(),
        )
        .add_named_argument(
            "arg2".to_string(),
            false,
            "arg2 input".to_string(),
        )
        .add_named_argument(
            "arg3".to_string(),
            false,
            "arg3 input".to_string(),
        )
        .add_positional_argument(
            "POS1".to_string(),
            true,
            "Positional argument 1".to_string(),
        )
        .add_positional_argument(
            "POS2".to_string(),
            false,
            "Positional argument 2".to_string(),
        );

    let parser = argparse::new_parser(
        arg_config,
        "This is an example app showing how to use argparse to parse commands"
            .to_string(),
    );

    parser.show_help();
}

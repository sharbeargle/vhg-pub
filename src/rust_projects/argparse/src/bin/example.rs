use argparse;

fn main() {
    let arg_config = argparse::new_argconfig()
        .add_flag('i', true, "Input file".to_string())
        .add_flag('o', false, "Output file".to_string())
        .add_argument("trace2".to_string(), true, true, "Take a tracer".to_string())
        .add_argument("trace4".to_string(), true, false, "Take a tracer".to_string())
        .add_argument("trace1".to_string(), false, true, "Take a trace".to_string())
        .add_argument("trace3".to_string(), false, false, "Take a trace".to_string())
        .add_flag_with_argument('x', "xylofone1".to_string(), true, true, "xylofone".to_string())
        .add_flag_with_argument('y', "xylofone2".to_string(), true, false, "xylofone".to_string())
        .add_flag_with_argument('z', "xylofone3".to_string(), false, true, "xylofone".to_string())
        .add_flag_with_argument('w', "xylofone4".to_string(), false, false, "xylofone".to_string())
        .add_positional_argument("FILENAME1".to_string(), true, "What file do you want to work on?".to_string())
        .add_positional_argument("FILENAME2".to_string(), false, "What file do you want to output to?".to_string());

    let parser = argparse::new_parser(arg_config, "This is an example app showing how to use argparse to parse commands. It is awesome.".to_string());

    parser.show_help();
}

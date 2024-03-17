use clap::{command, value_parser, Arg, ArgAction, Command, ValueHint};

pub fn new() -> Command {
    command!()
        .about("Simply convert images to ASCII art")
        .arg(
            Arg::new("input")
                .help("Input image file")
                .action(ArgAction::Set)
                .value_name("INPUT_FILE")
                .value_parser(value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath)
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Optional output text file - If omitted, writes to stdout")
                .action(ArgAction::Set)
                .value_name("FILE")
                .value_parser(value_parser!(std::path::PathBuf))
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("Output size in characters along the larger of the images dimensions")
                .action(ArgAction::Set)
                .value_name("INTEGER")
                .value_parser(value_parser!(u32))
                .default_value("25"),
        )
        .arg(
            Arg::new("x-stretch")
                .short('x')
                .long("x-stretch")
                .help("Factor to stretch the image by along its horizontal")
                .action(ArgAction::Set)
                .value_name("FLOAT")
                .value_parser(value_parser!(f64))
                .default_value("2.3"),
        )
        .arg(
            Arg::new("charset")
                .short('c')
                .long("charset")
                .help("Character set to use in ascending order of pixel value")
                .action(ArgAction::Set)
                .value_name("STRING")
                .value_parser(value_parser!(String))
                .default_value(" .:-=+*%#@"),
        )
}

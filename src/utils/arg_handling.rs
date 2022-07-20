use super::logging::Mode;
use std::env;

pub struct Args {
    pub verbosity: Mode,
}

fn print_verbosity(verbosity: &Mode) {
    let normal = String::from("\x1b[0m");
    let bold = String::from("\x1b[1m");
    println!(
        "\nVerbosity set to: {}{}{} \n",
        bold,
        verbosity.to_string(),
        normal
    );
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    let mut verbosity: Mode = Mode::Warning; // default
    let mut value: String;
    for (i, arg) in args.iter().enumerate() {
        if i < args_len - 1 {
            value = args[i + 1].to_lowercase();
            if arg == "-v" || arg == "--verbosity" {
                if value == "info" {
                    verbosity = Mode::Info;
                } else if value == "error" {
                    verbosity = Mode::Error;
                } else {
                    println!("Unknown verbosity level: {}", value);
                    println!("Setting verbosity to default.");
                }
            }
        }
    }
    print_verbosity(&verbosity);
    return Args { verbosity };
}

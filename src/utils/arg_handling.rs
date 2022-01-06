use std::env;

use super::logging::Logger;

pub fn handle_args() -> Logger {
    struct Mode {
        val: i8,
    }

    impl Mode {
        fn info(&self) {
            let mut out = String::from("OFF");
            let normal = String::from("\x1b[0m");
            let bold = String::from("\x1b[1m");
            let blinking = String::from("\x1b[5m");
            if self.val == 1 {
                out = String::from("ON")
            }
            println!("\nVerbose debug mode: {}{}{} \n", &bold, &out, &normal);
        }
    }

    let mut mode = Mode { val: 0 };

    // parse arguments
    let args: Vec<String> = env::args().collect();
    println!("\nArgument vector: {:?}", &args);

    for arg in &args {
        if arg == "debug" {
            mode.val = 1;
        }
    }
    mode.info();
    let logger = Logger { verbose: mode.val };

    logger
}

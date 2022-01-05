use std::env;

#[allow(dead_code)]
fn create_socket() {}

fn handle_args() {
    struct Debug {
        val: bool,
    }
    impl Debug {
        fn info(&self) {
            let mut out = String::from("OFF");
            let normal = String::from("\x1b[0m");
            let bold = String::from("\x1b[1m");
            #[allow(unused_variables)]
            let blinking = String::from("\x1b[5m");
            if self.val == true {
                out = String::from("ON")
            }
            println!("\nDebug mode: {}{}{} \n", &bold, &out, &normal)
        }
    }
    let mut mode = Debug { val: false };
    // parse arguments
    let args: Vec<String> = env::args().collect();
    println!("\nArgument vector: {:?}", &args);

    for arg in &args {
        if arg == "debug" {
            mode.val = true;
        }
    }
    mode.info()
}

fn main() {
    // handle those args biatch
    handle_args();
}

mod networking;
mod utils;

// standard
use std::thread::sleep;
use std::time::Duration;
// local modules
use networking::requests::{RequestSetup};
use utils::logging::Logger;
use utils::timing::Timing;
use utils::arg_handling::parse_args;

fn main() {
    let args = parse_args();
    let l = Logger {
        mode: args.verbosity,
    };

    let r = RequestSetup {
        host: "bla".to_string(),
        base_file_url: "bla".to_string(),
    };

    let mut t = Timing {
        start: None,
        duration: None,
    };

    t.start_timer();
    let sleeping_time = Duration::from_millis(1234);
    sleep(sleeping_time);
    t.end_timer();
    println!("{:?}", t.duration);
    println!("{:?}", t.get_timestamp());
    println!("{:?}", t.convert_to_milliseconds());
    // println!("{}", t.get_timestamp());
}

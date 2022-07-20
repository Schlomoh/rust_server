mod networking;
mod utils;

// standard
use std::thread::sleep;
use std::time::Duration;
// local modules
use networking::requests::RequestSetup;
use utils::arg_handling::parse_args;
use utils::logging::Logger;
use utils::timing::Timing;

fn main() {
    let args = parse_args();
    let l = Logger {
        mode: args.verbosity,
    };

    let t = Timing {
        start: None,
        duration: None,
    };

    let r = RequestSetup {
        host: "bla".to_string(),
        test_file_url: "bla".to_string(),
        timer: t,
        download_iteration: 50,
    };
}

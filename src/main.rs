mod networking;
mod utils;

use networking::listener::listen;
use utils::{arg_handling::handle_args};

fn main() {
    // handle those args biatch
    let logger = handle_args();

    // calling the listener function
    listen(logger)
}

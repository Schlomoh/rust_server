use std::net::TcpListener;

use super::super::utils::logging::Logger;


pub fn listen (logger: Logger) {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        logger.log(String::from("Connection established!"));
    }
}
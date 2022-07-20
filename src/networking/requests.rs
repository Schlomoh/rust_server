use crate::utils::timing::Timing;

use error_chain::error_chain;
use reqwest::blocking::Response;
use std::{io::Read, time::Duration};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

struct RequestResult {
    status_code: u16,
    body: String,
    duration: Option<Duration>,
}

fn make_request(url: &String) -> Result<RequestResult> {
    let mut t = Timing {
        start: None,
        duration: None,
    };
    let mut body = String::new();
    t.start_timer();
    let mut res = reqwest::blocking::get(url)?;
    res.read_to_string(&mut body)?;
    t.end_timer();

    Ok(RequestResult {
        status_code: res.status().as_u16(),
        body: body,
        duration: t.duration,
    })
}

pub struct RequestSetup {
    pub host: String,
    pub test_file_url: String,
    pub timer: Timing,
    pub download_iteration: u32,
}
impl RequestSetup {
    pub fn get_request_time(&mut self) -> u128 {
        let mut res: Result<RequestResult>;
        if self.download_iteration > 1 {
            for _ in 0..self.download_iteration {
                res = make_request(&self.test_file_url);
            }
        } else {
            res = make_request(&self.test_file_url);
        };
        return 23;
    }
}

use std::time::{Duration, Instant};

pub struct Timing {
    pub start: Option<Instant>,
    pub duration: Option<Duration>,
}

impl Timing {
    pub fn convert_to_milliseconds(&self) -> u128 {
        let dur_ms: u128;
        match self.duration {
            Some(d) => dur_ms = d.as_millis(),
            None => dur_ms = 0,
        };
        return dur_ms;
    }

    pub fn get_timestamp(&self) -> String {
        let dur_min: u64;
        let remaining_secs: u64;
        let remaining_millis: u32;
        match self.duration {
            Some(d) => {
                dur_min = d.as_secs() / 60;
                remaining_secs = d.as_secs() % 60;
                remaining_millis = d.subsec_millis();
                return format!("{}min {}s {}ms", dur_min, remaining_secs, remaining_millis);
            }
            None => return "0:0:0".to_string(),
        };
    }

    pub fn start_timer(&mut self) {
        match self.start {
            Some(x) => {
                self.start = Some(x);
            }
            None => {
                self.start = Some(Instant::now());
            }
        }
    }

    pub fn end_timer(&mut self) {
        match self.duration {
            Some(x) => {
                self.duration = Some(x);
            }
            None => match self.start {
                Some(x) => self.duration = Some(x.elapsed()),
                None => self.duration = None,
            },
        }
    }
}
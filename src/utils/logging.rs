use std::fmt;

#[derive(PartialEq, PartialOrd)] // for comparison
pub enum Mode {
    Info = 0,
    Warning = 1,
    Error = 2,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Info => write!(f, "INFO"),
            Mode::Warning => write!(f, "WARNING"),
            Mode::Error => write!(f, "ERROR"),
        }
    }
}

pub struct Logger {
    pub mode: Mode,
}

impl Logger {
    pub fn log(&self, message: &str, mode: Mode) {
        if mode == self.mode || mode >= self.mode {
            println!("{}: {}", mode.to_string(), message);
        }
    }
}

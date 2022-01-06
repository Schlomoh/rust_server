pub struct Logger {
    pub verbose: i8,
}

impl Logger {
    pub fn log(&self, msg: String) {
        if self.verbose  >= 1 {
            println!("{}", &msg)
        }
    }
}

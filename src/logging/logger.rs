use super::{Log, LogLevel};
use env::Env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::{stdout, Stdout};

pub struct Logger<W: Write> {
    out: W,
}

impl<W: Write> Logger<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

impl Default for Logger<File> {
    fn default() -> Self {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("log.txt")
            .unwrap();

        Self::new(file)
    }
}

impl Default for Logger<Stdout> {
    fn default() -> Self {
        Self::new(stdout())
    }
}

impl<W: Write> Log for Logger<W> {
    fn log(&mut self, log_level: LogLevel, msg: &str) {
        if log_level.is_on() && log_level >= Env::log_level() {
            writeln!(&mut self.out, "{}", msg).unwrap();
        }
    }
}

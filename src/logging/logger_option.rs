use super::{Log, LogLevel, Logger};
use std::convert::TryFrom;
use std::fs::{File, OpenOptions};
use std::io::{stdout, Error, Stdout};

pub enum LoggerOption {
    StdOut(Logger<Stdout>),
    File(Logger<File>),
    None,
}

impl Default for LoggerOption {
    fn default() -> Self {
        LoggerOption::StdOut(Logger::default())
    }
}

impl TryFrom<String> for LoggerOption {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        match from.to_lowercase().as_str() {
            "file" => {
                let file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("log.txt")?;

                Ok(LoggerOption::File(Logger::new(file)))
            }
            "stdout" => Ok(LoggerOption::StdOut(Logger::new(stdout()))),
            _ => Ok(LoggerOption::None),
        }
    }
}

impl TryFrom<&str> for LoggerOption {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Self::try_from(from.to_string())
    }
}

impl Log for LoggerOption {
    fn log(&mut self, level: LogLevel, msg: &str) {
        match self {
            LoggerOption::File(logger) => logger.log(level, msg),
            LoggerOption::StdOut(logger) => logger.log(level, msg),
            _ => {}
        }
    }
}

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Error as FmtError, Formatter};

pub enum LogLevel {
    All,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Off,
}

impl LogLevel {
    pub fn is_on(self) -> bool {
        !self.eq(&LogLevel::Off)
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Error
    }
}

impl Clone for LogLevel {
    fn clone(&self) -> Self {
        match self {
            LogLevel::All => LogLevel::All,
            LogLevel::Debug => LogLevel::Debug,
            LogLevel::Info => LogLevel::Info,
            LogLevel::Warn => LogLevel::Warn,
            LogLevel::Error => LogLevel::Error,
            LogLevel::Fatal => LogLevel::Fatal,
            LogLevel::Off => LogLevel::Off,
        }
    }
}

impl Copy for LogLevel {}

impl Eq for LogLevel {}

impl PartialEq for LogLevel {
    fn eq(&self, other: &LogLevel) -> bool {
        let lhs = *self as i32;
        let rhs = *other as i32;

        lhs.eq(&rhs)
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &LogLevel) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &LogLevel) -> Ordering {
        let lhs = *self as i32;
        let rhs = *other as i32;

        lhs.cmp(&rhs)
    }
}

pub struct LogLevelError;

impl Error for LogLevelError {}

impl Debug for LogLevelError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Logging level not recognized; must be in `all`, `debug`, `info`, `warn`, `error`, `fatal`, `off`")
    }
}

impl Display for LogLevelError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Logging level not recognized; must be in `all`, `debug`, `info`, `warn`, `error`, `fatal`, `off`")
    }
}

impl TryFrom<String> for LogLevel {
    type Error = LogLevelError;

    fn try_from(value: String) -> Result<LogLevel, LogLevelError> {
        match value.to_lowercase().as_str() {
            "all" => Ok(LogLevel::All),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "fatal" => Ok(LogLevel::Fatal),
            "off" => Ok(LogLevel::Off),
            _ => Err(LogLevelError),
        }
    }
}

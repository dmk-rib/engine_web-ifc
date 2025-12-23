//! Web-IFC Logging Helper

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::api::value::Value;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogLevel {
    LOG_LEVEL_DEBUG = 1,
    LOG_LEVEL_WARN = 3,
    LOG_LEVEL_ERROR = 4,
    LOG_LEVEL_OFF = 6,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            0 | 1 => LogLevel::LOG_LEVEL_DEBUG,
            2 | 3 => LogLevel::LOG_LEVEL_WARN,
            4 | 5 => LogLevel::LOG_LEVEL_ERROR,
            _ => LogLevel::LOG_LEVEL_OFF,
        }
    }
}

impl From<u8> for LogLevel {
    fn from(value: u8) -> Self {
        LogLevel::from(value as i32)
    }
}

static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::LOG_LEVEL_ERROR as usize);

pub struct Log;

pub enum LogArgs<'a> {
    Str(&'a [&'a str]),
    Value(&'a [Value]),
}

impl<'a> LogArgs<'a> {
    fn is_empty(&self) -> bool {
        match self {
            LogArgs::Str(values) => values.is_empty(),
            LogArgs::Value(values) => values.is_empty(),
        }
    }

    fn join(&self) -> String {
        match self {
            LogArgs::Str(values) => values.join(" "),
            LogArgs::Value(values) => values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

impl<'a> From<&'a [&'a str]> for LogArgs<'a> {
    fn from(values: &'a [&'a str]) -> Self {
        LogArgs::Str(values)
    }
}

impl<'a> From<&'a [Value]> for LogArgs<'a> {
    fn from(values: &'a [Value]) -> Self {
        LogArgs::Value(values)
    }
}

impl Log {
    pub fn set_log_level<L: Into<LogLevel>>(level: L) {
        LOG_LEVEL.store(level.into() as usize, Ordering::Relaxed);
    }

    pub fn log_values(msg: &str, values: &[Value]) {
        Self::log(msg, values);
    }

    pub fn log<'a, A: Into<LogArgs<'a>>>(msg: &str, args: A) {
        let args = args.into();
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_ERROR as usize {
            if args.is_empty() {
                println!("{msg}");
            } else {
                println!("{} {}", msg, args.join());
            }
        }
    }

    pub fn debug<'a, A: Into<LogArgs<'a>>>(msg: &str, args: A) {
        let args = args.into();
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_DEBUG as usize {
            if args.is_empty() {
                println!("DEBUG: {msg}");
            } else {
                println!("DEBUG: {} {}", msg, args.join());
            }
        }
    }

    pub fn warn<'a, A: Into<LogArgs<'a>>>(msg: &str, args: A) {
        let args = args.into();
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_WARN as usize {
            if args.is_empty() {
                eprintln!("WARN: {msg}");
            } else {
                eprintln!("WARN: {} {}", msg, args.join());
            }
        }
    }

    pub fn error<'a, A: Into<LogArgs<'a>>>(msg: &str, args: A) {
        let args = args.into();
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_ERROR as usize {
            if args.is_empty() {
                eprintln!("ERROR: {msg}");
            } else {
                eprintln!("ERROR: {} {}", msg, args.join());
            }
        }
    }
}

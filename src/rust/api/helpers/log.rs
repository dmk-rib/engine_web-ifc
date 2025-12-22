//! Web-IFC Logging Helper

use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogLevel {
    LOG_LEVEL_DEBUG = 1,
    LOG_LEVEL_WARN = 3,
    LOG_LEVEL_ERROR = 4,
    LOG_LEVEL_OFF = 6,
}

static LOG_LEVEL: AtomicUsize = AtomicUsize::new(LogLevel::LOG_LEVEL_ERROR as usize);

pub struct Log;

impl Log {
    pub fn set_log_level(level: LogLevel) {
        LOG_LEVEL.store(level as usize, Ordering::Relaxed);
    }

    pub fn log(msg: &str, args: &[&str]) {
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_ERROR as usize {
            if args.is_empty() {
                println!("{msg}");
            } else {
                println!("{} {}", msg, args.join(" "));
            }
        }
    }

    pub fn debug(msg: &str, args: &[&str]) {
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_DEBUG as usize {
            if args.is_empty() {
                println!("DEBUG: {msg}");
            } else {
                println!("DEBUG: {} {}", msg, args.join(" "));
            }
        }
    }

    pub fn warn(msg: &str, args: &[&str]) {
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_WARN as usize {
            if args.is_empty() {
                eprintln!("WARN: {msg}");
            } else {
                eprintln!("WARN: {} {}", msg, args.join(" "));
            }
        }
    }

    pub fn error(msg: &str, args: &[&str]) {
        if LOG_LEVEL.load(Ordering::Relaxed) <= LogLevel::LOG_LEVEL_ERROR as usize {
            if args.is_empty() {
                eprintln!("ERROR: {msg}");
            } else {
                eprintln!("ERROR: {} {}", msg, args.join(" "));
            }
        }
    }
}

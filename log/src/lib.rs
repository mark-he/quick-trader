use std::{fmt, str::FromStr};
use chrono::{Local, Utc};
use backtrace;

#[macro_export]
macro_rules! trace {
    () => {
        $crate::print("\n", $crate::Level::Trace)
    };
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*), $crate::Level::Trace);
    }};
}

#[macro_export]
macro_rules! debug {
    () => {
        $crate::print("\n", $crate::Level::Debug)
    };
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*), $crate::Level::Debug);
    }};
}

#[macro_export]
macro_rules! info {
    () => {
        $crate::print("\n", $crate::Level::Info)
    };
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*), $crate::Level::Info);
    }};
}

#[macro_export]
macro_rules! warn {
    () => {
        $crate::print("\n", $crate::Level::Warn)
    };
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*), $crate::Level::Warn);
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::print("\n", $crate::Level::Error)
    };
    ($($arg:tt)*) => {{
        $crate::print(&format!($($arg)*), $crate::Level::Error);
    }};
}

static mut LOGGER: Logger = Logger {level: Level::Warn, utc: true};
pub fn init(level: Level, utc: bool) {
    unsafe {
        LOGGER.utc = utc;
        LOGGER.level = level;
    }
}

pub fn print(message: &str, level: Level) {
    unsafe {
        LOGGER.print(message, level);
    }
}

#[derive(Debug, Clone)]
pub enum Level {
    Error = 16,
    Warn = 8,
    Info = 4,
    Debug = 2,
    Trace = 0,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant_name = match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };
        write!(f, "{}", variant_name)
    }
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ERROR" => Ok(Level::Error),
            "WARN" => Ok(Level::Warn),
            "INFO" => Ok(Level::Info),
            "DEBUG" => Ok(Level::Debug),
            "TRACE" => Ok(Level::Trace),
            _ => Ok(Level::Trace),
        }
    }
}

struct Logger {
    pub utc: bool,
    pub level: Level,
}

impl Logger {
    pub fn format(&self, message: &str, level: &Level) -> String {
        let time_str;
        if self.utc {
            time_str = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        } else {
            time_str = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        }
        format!("{} {}: {}", level, time_str, message)
    }

    pub fn print(&self, message: &str, level: Level) {
        if self.level.clone() as u32 <= level.clone() as u32 {
            let mut output = String::new();
            output.push_str(&format!("{}\n", message));
            match level {
                Level::Error => {
                    let backtrace = backtrace::Backtrace::new();
                    for frame in backtrace.frames() {
                        output.push_str(&format!("{:?}\n", frame));
                    }
                },
                _ => {},
            }
            print!("{}", self.format(&output, &level));
        }
    }
}
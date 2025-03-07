use chrono::Local;
use std::fmt::Display;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn log(level: LogLevel, module: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_message = format!("[{timestamp}] {level} - {module}: {message}\n");

        let log_dir = "logs";
        if !Path::new(log_dir).exists() {
            if let Err(e) = create_dir_all(log_dir) {
                eprintln!("Failed to create log directory: {e}");
                return;
            }
        }

        let log_file = format!("{log_dir}/application.log");
        let mut file = match OpenOptions::new().create(true).append(true).open(&log_file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open log file: {e}");
                return;
            }
        };

        if let Err(e) = file.write_all(log_message.as_bytes()) {
            eprintln!("Failed to write to log file: {e}");
        }
    }

    pub fn info(module: &str, message: &str) {
        Self::log(LogLevel::INFO, module, message);
    }

    pub fn warn(module: &str, message: &str) {
        Self::log(LogLevel::WARN, module, message);
    }

    pub fn error(module: &str, message: &str) {
        Self::log(LogLevel::ERROR, module, message);
    }

    pub fn debug(module: &str, message: &str) {
        if cfg!(debug_assertions) {
            Self::log(LogLevel::DEBUG, module, message);
        }
    }
}

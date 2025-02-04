//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    /// This function enables logging.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// config.enable();
    /// ```
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// This function disables logging.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// config.disable();
    /// ```
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// This function sets the log level.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// config.set_level(LogLevel::Debug);
    /// ```
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    /// This function gets the log level.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let config = Logging::new();
    /// let level = config.get_level();
    /// ```
    pub fn get_level(&self) -> LogLevel {
        self.level
    }

    /// This function sets the log output destination.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// config.set_destination(LogOutput::File("output.log".to_string()));
    /// ```
    /// 
    /// Send the log output to a file:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let mut config = Logging::new();
    /// config.set_destination(LogOutput::File("output.log".to_string()));
    /// ```
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }

    /// This function gets the log output destination.
    /// # Examples:
    /// ```
    /// use cli_utils::config::{Logging, LogLevel, LogOutput};
    /// let config = Logging::new();
    /// let destination = config.get_destination();
    /// ```
    pub fn get_destination(&self) -> String {
        match self.destination {
            LogOutput::File(ref path) => {
                println!("Logging to file: {}", path);
                path.to_string()
            }
            LogOutput::Stderr => {
                println!("Logging to stderr, the standard error stream");
                "stderr".to_string()
            },
            LogOutput::Stdout => {
                println!("Logging to stdout, the standard output stream");
                "stdout".to_string()
            },
        }
    }


}
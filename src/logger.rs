use super::level::Level;

use chrono::format::DelayedFormat;
use chrono::format::StrftimeItems;
use chrono::prelude::DateTime;
use chrono::prelude::Local;

/// *Logger*
///
/// Logging is a means of tracking events that happen when some software runs.
/// The software’s developer adds logging calls to their code to indicate that certain events have occurred.
/// An event is described by a descriptive message which can optionally contain variable data
/// (i.e. data that is potentially different for each occurrence of the event).
/// Events also have an importance which the developer ascribes to the event;
/// the importance can also be called the level or severity.
///
/// # Examples
/// ```
/// let logger = Logger::new();
///
/// logger.debug("Hello, world!");
/// logger.info("Hello, world!");
/// logger.warn("Hello, world!");
/// logger.error("Hello, world!");
/// logger.critical("Hello, world!");
/// ```
pub struct Logger {
    /// *name*
    ///
    /// Logger name.
    pub name: String,
}

impl Logger {
    /// *new*
    ///
    /// Create a new logger.
    ///
    /// # Arguments:
    ///
    /// * `name` - Logger name.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Logger {
    /// *log*
    ///
    /// # Arguments:
    ///
    /// * `level` - Log Level
    /// * `message` - Log message
    fn log(&self, level: Level, message: &str) {
        let fmt: &str = "%H:%M:%S %Y/%m/%d";

        let local: DateTime<Local> = Local::now();
        let timestamp: DelayedFormat<StrftimeItems> = local.format(fmt);

        let logger: String = self.name.clone();
        let level: String = level.name.clone();

        println!("{logger} {level} [{timestamp}] {message}");
    }

    /// *debug*
    ///
    /// Detailed information, typically of
    /// interest only when diagnosing problems.
    ///
    /// # Arguments:
    ///
    /// * `message` - Debug log message.
    ///
    /// # Examples:
    /// Report events that occur during normal operation
    /// of a program (e.g. for status monitoring or fault investigation)
    /// ```
    /// let logger = Logger::new();
    ///
    /// logger.debug("Hello, world!");
    /// ```
    pub fn debug(&self, message: &str) {
        let level: Level = Level::new("DEBUG");

        self.log(level, message);
    }

    /// *info*
    ///
    /// Confirmation that things are working as expected.
    ///
    /// # Arguments:
    ///
    /// * `message` - Info log message.
    ///
    /// # Examples:
    /// Report events that occur during normal operation
    /// of a program (e.g. for status monitoring or fault investigation).
    /// ```
    /// let logger = Logger::new();
    ///
    /// logger.info("Hello, world!");
    /// ```
    pub fn info(&self, message: &str) {
        let level: Level = Level::new("INFO");

        self.log(level, message);
    }

    /// *warn*
    ///
    /// An indication that something unexpected happened,
    /// or indicative of some problem in the near future.
    /// The software is still working as expected.
    ///
    /// # Arguments:
    ///
    /// * `message` - Success log message
    ///
    /// # Examples:
    /// Issue a warning regarding a particular runtime event.
    /// ```
    /// let logger = Logger::new();
    ///
    /// logger.warn("Hello, world!");
    /// ```
    pub fn warn(&self, message: &str) {
        let level: Level = Level::new("WARN");

        self.log(level, message);
    }

    /// *error*
    ///
    /// Due to a more serious problem,
    /// the software has not been
    /// able to perform some function.
    ///
    /// # Arguments:
    ///
    /// * `message` - Error log message.
    ///
    /// # Examples:
    /// Report suppression of an error without raising an exception
    /// (e.g. error handler in a long-running server process).
    /// ```
    /// let logger = Logger::new();
    ///
    /// logger.error("Hello, world!");
    /// ```
    pub fn error(&self, message: &str) {
        let level: Level = Level::new("ERROR");

        self.log(level, message);
    }

    /// *critical*
    ///
    /// A serious error, indicating that the program
    /// itself may be unable to continue running.
    ///
    /// # Arguments:
    ///
    /// * `message` - Critical log message.
    ///
    /// # Examples:
    /// Report suppression of an error without raising an exception
    /// (e.g. error handler in a long-running server process).
    /// ```
    /// let logger = Logger::new();
    ///
    /// logger.critical("Hello, world!");
    /// ```
    pub fn critical(&self, message: &str) {
        let level: Level = Level::new("CRITICAL");

        self.log(level, message);
    }
}

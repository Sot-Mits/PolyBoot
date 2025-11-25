use atty::Stream;
use colored::Colorize;
use fern::Dispatch;
use log::Level;
use std::fmt;
use std::io::{self};

//Trying to keep this as close as I can to original C implementation
pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let stdout_isatty = atty::is(Stream::Stdout);
    let stderr_isatty = atty::is(Stream::Stderr);

    let formatter =
        move |out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::Record| {
            let use_color = match record.level() {
                Level::Error | Level::Warn => stderr_isatty,
                _ => stdout_isatty,
            };

            let status = match record.level() {
                Level::Error => "[ERROR]:",
                Level::Warn => "[WARN]:",
                Level::Info => "[INFO]:",
                Level::Debug => "[DEBUG]:",
                Level::Trace => "[TRACE]:",
            };

            let status = if use_color {
                match record.level() {
                    Level::Error => status.red().bold().to_string(),
                    Level::Warn => status.yellow().bold().to_string(),
                    Level::Info => status.green().bold().to_string(),
                    Level::Debug => status.blue().bold().to_string(),
                    Level::Trace => status.white().bold().to_string(),
                }
            } else {
                status.to_string()
            };

            out.finish(format_args!("{} {}", status, message));
        };

    let stdout_dispatch = Dispatch::new()
        .filter(|metadata| matches!(metadata.level(), Level::Info | Level::Debug))
        .chain(io::stdout());

    let stderr_dispatch = Dispatch::new()
        .filter(|metadata| matches!(metadata.level(), Level::Warn | Level::Error))
        .chain(io::stderr());

    Dispatch::new()
        .format(formatter)
        .level(log::LevelFilter::Trace)
        .chain(stdout_dispatch)
        .chain(stderr_dispatch)
        .apply()?;

    Ok(())
}

//Wrappers for logging functions
pub fn _log_info(args: fmt::Arguments) {
    log::info!("{}", args);
}
pub fn _log_warn(args: fmt::Arguments) {
    log::warn!("{}", args);
}
pub fn _log_error(args: fmt::Arguments) {
    log::error!("{}", args);
}
pub fn _log_debug(args: fmt::Arguments) {
    log::debug!("{}", args);
}

//Macros for wrappers
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::_log_info(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::_log_warn(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::_log_error(format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::_log_debug(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => {{
        //bring required items into scope for the macro expansion
        use std::io::{self, Write};
        use atty::Stream;
        use colored::Colorize;

        let prompt = format!($($arg)*);
        let is_tty = atty::is(Stream::Stdout);
        if is_tty {
            print!("{}", "[INPUT]:".cyan().bold());
        } else {
            print!("[INPUT]:");
        }
        print!(" {} ", prompt);

        //attempt to flush; return Err if flush fails
        if let Err(e) = io::stdout().flush() {
            Err(e)
        } else {
            let mut s = String::new();
            match io::stdin().read_line(&mut s) {
                Ok(_) => Ok(s.trim_end_matches(&['\n', '\r'][..]).to_string()),
                Err(e) => Err(e),
            }
        }
    }};
}

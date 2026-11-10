//! CLI logging setup with stderr output and rotating local log files.

use std::env;
use std::error::Error;
use std::fs;
use std::io::IsTerminal as _;
use std::path::PathBuf;

use tracing_appender::non_blocking::NonBlockingBuilder;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

type InitResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

/// Maximum number of log events buffered before the application waits for the file writer.
const LOG_BUFFERED_LINES_LIMIT: usize = 1024;

/// Installs stderr and daily-file tracing layers, returning the guard that flushes file events.
pub(crate) fn init() -> InitResult<Option<WorkerGuard>> {
    let filter = env_filter();
    let Some(directory) = log_directory() else {
        eprintln!("no log directory is available; continuing with stderr-only tracing");
        init_stderr_only(filter)?;
        return Ok(None);
    };

    if let Err(error) = fs::create_dir_all(&directory) {
        eprintln!(
            "could not create log directory {}: {error}; continuing with stderr-only tracing",
            directory.display()
        );
        init_stderr_only(filter)?;
        return Ok(None);
    }

    let appender = match rolling::RollingFileAppender::builder()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix(env!("CARGO_PKG_NAME"))
        .filename_suffix("log")
        .max_log_files(7)
        .build(&directory)
    {
        Ok(appender) => appender,
        Err(error) => {
            eprintln!(
                "could not initialize rotating logs in {}: {error}; continuing with stderr-only tracing",
                directory.display()
            );
            init_stderr_only(filter)?;
            return Ok(None);
        }
    };

    let (file_writer, guard) = NonBlockingBuilder::default()
        .buffered_lines_limit(LOG_BUFFERED_LINES_LIMIT)
        .lossy(false)
        .finish(appender);
    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal());
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(stderr_layer)
        .with(file_layer)
        .try_init()?;

    Ok(Some(guard))
}

fn init_stderr_only(filter: EnvFilter) -> InitResult<()> {
    let stderr_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal());

    tracing_subscriber::registry()
        .with(filter)
        .with(stderr_layer)
        .try_init()?;

    Ok(())
}

fn env_filter() -> EnvFilter {
    let Some(directives) = env::var_os("RUST_LOG") else {
        return EnvFilter::new("info");
    };
    let Some(directives) = directives.to_str() else {
        eprintln!("RUST_LOG is not valid UTF-8; using the info log level");
        return EnvFilter::new("info");
    };

    if let Ok(filter) = EnvFilter::try_new(directives) {
        filter
    } else {
        eprintln!("RUST_LOG is invalid; using the info log level");
        EnvFilter::new("info")
    }
}

fn log_directory() -> Option<PathBuf> {
    if let Some(directory) = env::var_os("APP_LOG_DIR").filter(|value| !value.is_empty()) {
        return Some(PathBuf::from(directory));
    }

    default_log_directory()
}

#[cfg(windows)]
fn default_log_directory() -> Option<PathBuf> {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|base| base.join(env!("CARGO_PKG_NAME")).join("logs"))
}

#[cfg(target_os = "macos")]
fn default_log_directory() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|home| {
            home.join("Library")
                .join("Logs")
                .join(env!("CARGO_PKG_NAME"))
        })
}

#[cfg(not(any(windows, target_os = "macos")))]
fn default_log_directory() -> Option<PathBuf> {
    env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            env::var_os("HOME")
                .map(PathBuf::from)
                .filter(|home| home.is_absolute())
                .map(|home| home.join(".local").join("state"))
        })
        .map(|base| base.join(env!("CARGO_PKG_NAME")).join("logs"))
}

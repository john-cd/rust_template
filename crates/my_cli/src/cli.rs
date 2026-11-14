//! Parses command-line arguments and handles greeting output and process errors.
//!
//! TODO Reviewed ??-??-2026.

use std::io::Write;
use std::process::ExitCode;

use clap::Parser;
use tracing::error;
use tracing::info;

/// Starter command-line application for the `my_cli` workspace.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Name to include in the greeting.
    #[arg(default_value = "world")]
    name: String,
}

/// Parse command-line arguments and print the greeting.
///
/// # Returns
/// A process status reflecting the outcome of the command.
pub fn run() -> ExitCode {
    let args = Args::parse();
    let _log_guard = match super::logging::init() {
        Ok(guard) => guard,
        Err(error) => {
            eprintln!("could not initialize tracing: {error}");
            return ExitCode::FAILURE;
        }
    };

    info!(command = "greet", "command started");
    let stdout = std::io::stdout();
    match write_greeting(stdout.lock(), &args.name) {
        Ok(()) => {
            info!(command = "greet", "command completed");
            ExitCode::SUCCESS
        }
        Err(error) => {
            let _ = report_greeting_error(
                &error,
                tracing::enabled!(tracing::Level::ERROR),
                std::io::stderr().lock(),
            );
            ExitCode::FAILURE
        }
    }
}

/// Reports a failed stdout write through tracing or directly to stderr when filtered out.
///
/// # Arguments
/// * `error` - Underlying failure whose details are preserved or converted into the returned error.
/// * `tracing_enabled` - Bool carrying the tracing enabled information consumed by the cli workflow.
/// * `fallback_stderr` - Impl write carrying the fallback stderr information consumed by the cli workflow.
///
/// # Returns
/// The successful result value produced by this operation.
///
/// # Errors
/// Returns an error if a fallible operation cannot complete.
fn report_greeting_error(
    error: &std::io::Error,
    tracing_enabled: bool,
    mut fallback_stderr: impl Write,
) -> std::io::Result<()> {
    if tracing_enabled {
        error!(%error, command = "greet", operation = "write_stdout", "could not write greeting");
        Ok(())
    } else {
        writeln!(fallback_stderr, "could not write greeting: {error}")
    }
}

/// Persists the greeting produced by the cli workflow so later operations can retrieve it.
///
/// # Arguments
/// * `writer` - Destination that receives the serialized or formatted result.
/// * `name` - Human-readable value used to identify or display the selected cli.
///
/// # Returns
/// The successful result value produced by this operation.
///
/// # Errors
/// An error is returned when the write greeting operation cannot complete.
fn write_greeting(mut writer: impl Write, name: &str) -> std::io::Result<()> {
    writeln!(writer, "{}", my_lib::greeting(name))
}

/// Exercises output handling when the destination writer fails.
#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::report_greeting_error;
    use super::write_greeting;

    /// Writer double that always reports a broken pipe.
    struct BrokenWriter;

    impl Write for BrokenWriter {
        /// Simulates a closed output stream.
        ///
        /// # Arguments
        /// * `_buffer` - [u8] carrying the buffer information consumed by the broken writer workflow.
        ///
        /// # Returns
        /// The successful result value produced by this operation.
        ///
        /// # Errors
        /// Returns an error if std::io::ErrorKind::BrokenPipe.into( when a request is rejected.
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }
        /// Accepts flushes so the simulated write failure stays focused.
        ///
        /// # Returns
        /// The successful result value produced by this operation.
        ///
        /// # Errors
        /// Returns an error if a fallible operation cannot complete.

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    /// Propagates a broken-pipe error from the output writer.

    #[test]
    fn write_greeting_returns_output_errors_to_the_caller() {
        let error = write_greeting(BrokenWriter, "Alice").expect_err("writer is closed");
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }

    /// Keeps the process-boundary diagnostic visible when the event filter is disabled.
    #[test]
    fn output_errors_fall_back_to_stderr_when_tracing_is_disabled() {
        let error = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "closed pipe");
        let mut stderr = Vec::new();

        report_greeting_error(&error, false, &mut stderr).expect("fallback stderr write succeeds");

        assert_eq!(
            stderr, b"could not write greeting: closed pipe\n",
            "the error remains visible when RUST_LOG filters out errors"
        );
    }
}

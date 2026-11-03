use std::io::Write;
use std::process::ExitCode;

use clap::Parser;

/// Starter command-line application for the `my_cli` workspace.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Name to include in the greeting.
    #[arg(default_value = "world")]
    name: String,
}

/// Parse command-line arguments and print the greeting.
#[must_use]
pub fn run() -> ExitCode {
    let args = Args::parse();
    let stdout = std::io::stdout();
    match write_greeting(stdout.lock(), &args.name) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("could not write greeting: {error}");
            ExitCode::FAILURE
        }
    }
}

fn write_greeting(mut writer: impl Write, name: &str) -> std::io::Result<()> {
    writeln!(writer, "{}", my_lib::greeting(name))
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::write_greeting;

    struct BrokenWriter;

    impl Write for BrokenWriter {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_greeting_returns_output_errors_to_the_caller() {
        let error = write_greeting(BrokenWriter, "Alice").expect_err("writer is closed");
        assert_eq!(error.kind(), std::io::ErrorKind::BrokenPipe);
    }
}

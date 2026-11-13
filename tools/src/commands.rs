//! Defines development subcommands and runs the matching repository tools.
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;
use std::process::Command;
use std::process::ExitCode;

use clap::Parser;
use clap::Subcommand;

/// Development commands for this checkout.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// The development task selected by the caller.
    #[command(subcommand)]
    command: Task,
}

/// Supported maintenance and local development tasks.
#[derive(Debug, Subcommand)]
enum Task {
    /// Run all local CI checks (requires the tools listed in CONTRIBUTING.md).
    Check,
    /// Format Rust and repository configuration/documentation.
    Format,
    /// Build the Zola website, or preview it locally.
    Website {
        /// Start the preview server instead of building static output.
        #[arg(long)]
        serve: bool,
    },
}

/// Parse and run the selected development command.
pub fn run() -> ExitCode {
    let args = Args::parse();
    // Cargo sets this path for the checkout that builds this development-only binary.
    let Some(root) = Path::new(env!("CARGO_MANIFEST_DIR")).parent() else {
        eprintln!("could not locate the workspace root");
        return ExitCode::FAILURE;
    };
    let (program, arguments) = command_for(&args.command);
    let mut command = Command::new(program);
    command.args(arguments).current_dir(root);
    if program == "zola" {
        command.env("RUST_LOG", "error");
    }
    match command.status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(status) => {
            eprintln!("{program} failed with {status}");
            ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("could not run {program}: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Maps a parsed task to the executable and arguments that implement it.
fn command_for(task: &Task) -> (&'static str, &'static [&'static str]) {
    match task {
        Task::Check => ("cargo", &["make", "--quiet", "ci"]),
        Task::Format => ("cargo", &["make", "--quiet", "format"]),
        Task::Website { serve: false } => ("zola", &["--root", "website", "build"]),
        Task::Website { serve: true } => ("zola", &["--root", "website", "serve"]),
    }
}

/// Checks the mapping between development tasks and their commands.
#[cfg(test)]
mod tests {
    use super::Task;
    use super::command_for;

    /// Maps each supported task to the expected executable and arguments.
    #[test]
    fn command_mapping_covers_each_task() {
        assert_eq!(
            command_for(&Task::Check),
            ("cargo", &["make", "--quiet", "ci"][..])
        );
        assert_eq!(
            command_for(&Task::Format),
            ("cargo", &["make", "--quiet", "format"][..])
        );
        assert_eq!(
            command_for(&Task::Website { serve: false }),
            ("zola", &["--root", "website", "build"][..])
        );
        assert_eq!(
            command_for(&Task::Website { serve: true }),
            ("zola", &["--root", "website", "serve"][..])
        );
    }
}

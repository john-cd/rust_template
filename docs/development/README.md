<!-- TODO Reviewed ??-??-???? -->

# Development

## Setup

Install the Rust toolchain and tools listed in the root [README](../../README.md). The repository pins its Rust channel in `rust-toolchain.toml`.

## Workflow

1. Create a feature branch from `develop`.
2. Make focused changes in the relevant workspace crate.
3. Run the applicable `cargo make` checks.
4. Integrate reviewed work into `develop`.

Cargo Make task definitions are in the root `Makefile.toml`; CI runs the equivalent formatting, build, lint, test, doctest, and dependency checks.

See [Configuration](configuration.md) for a file-by-file guide to the workspace settings.

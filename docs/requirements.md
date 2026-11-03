# Requirements

This document records the requirements for adopting and extending the Rust workspace template.
Replace these starter requirements with application-specific requirements when creating a project.

## Functional requirements

- The workspace can contain reusable library crates under `crates/`.
- A command-line application can parse arguments with Clap and call the reusable library.
- The `tools` package provides repository-local formatting, validation, and website commands.
- Contributors can run formatting, checks, Clippy, tests, documentation, and dependency policy checks
  through the documented Cargo Make tasks.

## Non-functional requirements

- Workspace crates inherit shared package metadata, dependencies, and lint policy from the root manifest.
- Library APIs and modules have Rust documentation, and documentation builds with warnings denied.
- Local development and CI use the same pinned Rust toolchain and locked dependency graph.
- Build artifacts and compiler cache are kept outside tracked source files.

## Constraints

- The workspace uses Rust edition 2024 and declares Rust 1.85 as its minimum supported version.
- The project is licensed under MIT OR Apache-2.0 and is not published by default.
- The full local CI task requires the external tools listed in `CONTRIBUTING.md`.

## Assumptions

- Application-specific behavior, package names, and deployment requirements are defined by the project adopting this template.
- Website tasks require Zola; dependency policy checks require `cargo-deny`.

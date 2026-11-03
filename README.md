<!-- TODO Reviewed ??-??-???? -->

# Rust workspace starter

A Rust workspace starter with a sample library, a Clap-based binary, developer tools, shared Cargo policy, a Docker Compose devcontainer, cross-platform CI, and a Zola website.

## Branch workflow

Use `develop` as the integration branch. Integrate reviewed work there, then promote a release to `main` when it is ready. Create feature branches from `develop`.

## Workspace layout

```text
.
├── crates/
│   ├── my_lib/           # Sample library crate
│   └── my_cli/           # Sample Clap binary
├── docs/                 # Project documentation
├── scripts/              # Repository maintenance and development scripts
├── tools/                # Workspace development binary (cargo tools)
├── website/              # Zola content, templates, and static assets
├── .cargo/               # Cargo configuration
├── .config/              # cargo-nextest configuration
├── .devcontainer/        # Docker Compose development environment
└── .github/workflows/    # CI workflows
```

Add workspace packages under `crates/`. Shared package metadata and lint policy live in the root `Cargo.toml`; crate manifests can inherit those values with `workspace = true`.

The library crate starts with `config`, `error`, `models`, and `services` modules. The companion CLI uses Clap derive and calls the library greeting: `cargo run -p my_cli -- Alice`.

## Prerequisites

- Rust 1.99.1 with `clippy` and `rustfmt`; rustup installs the pinned toolchain from `rust-toolchain.toml`.
- `cargo-binstall` and `sccache`; Cargo uses `sccache` as the compiler wrapper for Rust builds. See [CONTRIBUTING.md](CONTRIBUTING.md) for pinned install commands.
- [`cargo-make`](https://github.com/sagiegurari/cargo-make)
- [`cargo-nextest`](https://nexte.st/)
- [`cargo-deny`](https://embarkstudios.github.io/cargo-deny/)
- dprint, lychee, and Zola; see [CONTRIBUTING.md](CONTRIBUTING.md) for installation and pinned versions.

## Common commands

```sh
cargo make --quiet fmt       # Check formatting
cargo make --quiet check     # Check all workspace targets
cargo make --quiet clippy    # Run Clippy with warnings denied
cargo make --quiet test      # Run tests with nextest
cargo make --quiet doctest   # Run documentation tests
cargo make --quiet deny      # Check dependency policy
cargo make --quiet ci        # Run the complete local CI task set
cargo tools format   # Format Rust, documentation, and configuration
cargo tools website  # Build the Zola website
```

The CI task uses the `ci` Cargo profile. Additional `debugging` and `tools` profiles are available with `cargo build --profile debugging` and `cargo build --profile tools`.

## Devcontainer

Open the repository in VS Code and choose **Reopen in Container**. The devcontainer builds from `.devcontainer/Dockerfile` with Docker Compose on Debian Trixie, mounts the workspace, and installs the Rust Analyzer and TOML extensions. Edit `.devcontainer/.env` to configure its Rust and development-tool versions.
It also installs the pinned Rust formatter and development tools. Port 1111 is forwarded for Zola previews.

## Documentation

Start a new project with the [template adoption guide](docs/guides/adopting-template.md).
Shared VS Code settings/tasks and optional `prek` hooks provide the same checks as
the command-line workflow.

Start at [`docs/README.md`](docs/README.md). See the [configuration reference](docs/development/configuration.md) for the purpose of each repository configuration file. Add architecture notes, development instructions, and task guides in the corresponding section.

## Contributing and license

Read [CONTRIBUTING.md](CONTRIBUTING.md) for setup, checks, and template adoption.
The root includes the contributor guide and dual-license texts. Crate READMEs link to the shared contributor guide and license; package-level license-file inclusion should be checked before publishing a crate.
Licensed under MIT OR Apache-2.0; see [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT), and
[LICENSE-APACHE](LICENSE-APACHE).

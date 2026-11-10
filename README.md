<!-- TODO Reviewed ??-??-???? -->

# Rust workspace starter

A Rust workspace starter with a sample library, a Clap-based binary, developer tools, shared Cargo policy, a Docker Compose devcontainer, cross-platform CI, and a Zola website.

Create a project with `cargo-generate` 0.24 or newer:

```sh
cargo generate --git https://github.com/john-cd/rust_template.git --branch develop --name myproject
```

The generated workspace names its CLI package after the selected project name
and its library package `<project-name>_lib`. Follow the [adoption guide](docs/guides/adopting-template.md)
to replace the sample behavior and metadata.

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
├── tools/                # Stub for project-specific development and compile-time tools
├── website/              # Zola content, templates, and static assets
├── .cargo/               # Cargo configuration
├── .config/              # cargo-nextest configuration
├── .devcontainer/        # Docker Compose development environment
└── .github/workflows/    # CI workflows
```

Add workspace packages under `crates/`. Shared package metadata and lint policy live in the root `Cargo.toml`; crate manifests can inherit those values with `workspace = true`.

`tools/` is a starting point for project-specific development-only and compile-time utilities,
such as code generators, schema checks, or build helpers. Extend or replace the sample `cargo tools`
commands as needed; keep application runtime behavior in the application crates.

The library crate starts with `config`, `error`, `models`, and `services` modules. The companion CLI uses Clap derive and calls the library greeting: `cargo run -p {{project-name}} -- Alice`.

## Prerequisites

- Rust 1.99.1 with `clippy` and `rustfmt`; rustup installs the pinned toolchain from `rust-toolchain.toml`.
- `cargo-binstall` and `sccache`; Cargo uses `sccache` as the compiler wrapper for Rust builds. See [CONTRIBUTING.md](CONTRIBUTING.md) for pinned install commands.
- [`cargo-make`](https://github.com/sagiegurari/cargo-make)
- [`cargo-nextest`](https://nexte.st/)
- [`cargo-deny`](https://embarkstudios.github.io/cargo-deny/)
- [`cargo-machete`](https://github.com/bnjbvr/cargo-machete), [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov), and [`zizmor`](https://zizmor.sh/) for optional dependency, coverage, and workflow checks.
- dprint, lychee, and Zola; see [CONTRIBUTING.md](CONTRIBUTING.md) for installation and pinned versions.

## Common commands

```sh
cargo make --quiet f         # Apply Rust and repository formatting
cargo make --quiet fc        # Check Rust and repository formatting
cargo make --quiet fmt       # Check Rust formatting
cargo make --quiet check     # Check all workspace targets
cargo make --quiet c         # Short alias for check
cargo make --quiet build     # Build all workspace targets
cargo make --quiet b         # Short alias for build
cargo make --quiet clippy    # Run Clippy with warnings denied
cargo make --quiet l         # Short alias for clippy
cargo make --quiet fix       # Apply compiler suggestions
cargo make --quiet lint-fix  # Apply Clippy suggestions
cargo make --quiet run       # Run the sample CLI (alias: r)
cargo make --quiet release   # Build the release profile
cargo make --quiet test      # Run tests with nextest (alias: t)
cargo make --quiet bench     # Run benchmarks (bench-check compiles only)
cargo make --quiet doctest   # Run documentation tests
cargo make --quiet deny      # Check dependency policy
cargo make --quiet machete   # Review potentially unused dependencies
cargo make --quiet coverage  # Measure test coverage
cargo make --quiet coverage-html # Generate an HTML coverage report
cargo make --quiet zizmor    # Audit GitHub Actions workflows
cargo make --quiet pre       # Pre-commit formatting, check, and Clippy
cargo make --quiet doc       # Build API documentation
cargo make --quiet web       # Build the website
cargo make --quiet hooks-install # Install prek Git hooks
cargo make --quiet hooks-run # Run prek against staged files
cargo make --quiet build-env # Show Cargo output and cache environment
cargo make --quiet docker-build-dev  # Build the development image
cargo make --quiet docker-build-prod # Build the production image
cargo make --quiet ci        # Run the complete local CI task set
cargo tools format   # Format Rust, documentation, and configuration
cargo tools website  # Build the Zola website
```

The CI task uses the `ci` Cargo profile. The `debug` profile is available with `cargo build --profile debug`; the `cargo tools` alias builds the developer binary with the `tools` profile.
`Makefile.toml` owns these Cargo Make tasks; `cargo tools format` and `cargo tools check` are
convenience commands that delegate to Cargo Make, while `cargo tools website` invokes Zola.

Cargo writes build output to the checkout-relative `target/` directory by default. Set
`CARGO_TARGET_DIR` to use an external build directory, `SCCACHE_DIR` to relocate the sccache
cache, or `SCCACHE_BASEDIRS` to share cache entries across checkout paths. The shared-cache
worktree slots are `.worktrees/a`, `.worktrees/b`, and `.worktrees/c`.

On Windows, Cargo Make defaults `CARGO_TARGET_DIR` to `D:/target/<project>` and
`CARGO_BUILD_BUILD_DIR` to `C:/code_build/<project>`. Set `CARGO_TARGET_ROOT` or
`CARGO_BUILD_ROOT` to change those roots; explicit `CARGO_TARGET_DIR` and
`CARGO_BUILD_BUILD_DIR` values take precedence. Other platforms keep the direct-Cargo defaults.

## Devcontainer

To build the deployment image from a POSIX shell, load the shared image and tool versions:

```sh
set -a
. .devcontainer/.env
set +a
docker build -f .devcontainer/Dockerfile --target production \
  --build-arg RUST_VERSION --build-arg DEBIAN_VERSION --build-arg CARGO_BINSTALL_VERSION \
  --build-arg CARGO_CHEF_VERSION \
  --build-arg CARGO_PACKAGE_NAME={{project-name}} -t {{project-name}} .
docker run --rm {{project-name}} Alice
```

The Cargo package names and production executable use the generated project name.

Open the repository in VS Code and choose **Reopen in Container**. The devcontainer builds from `.devcontainer/Dockerfile` with Docker Compose on Debian Trixie, mounts the workspace, and installs the Rust Analyzer and TOML extensions. Edit `.devcontainer/.env` to configure its project prefix, production executable name, Rust version, and development-tool versions.
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

<!-- TODO Reviewed ??-??-???? -->

# Rust workspace starter

A Rust workspace starter with a sample library, a Clap-based binary, developer tools, shared Cargo policy, a Docker Compose devcontainer, cross-platform CI, and a Zola website.

Create a project with `cargo-generate` 0.24 or newer:

```sh
cargo generate --git https://github.com/john-cd/rust_template.git --branch develop --name myproject
```

Replace `myproject` with your project name. The template currently uses `develop` as the integration branch; the `main` branch is reserved for releases.

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

For local development, install Git, Rust through [rustup](https://rust-lang.org/tools/install/), and
the repository tools. On Windows, install the MSVC C++ build tools if rustup prompts for them.
The checked-in toolchain files pin Rust 1.99.1 for development and nightly rustfmt for formatting;
the declared MSRV is 1.98.1 and CI checks it separately.

After cloning the repository, install both Rust toolchains:

```sh
rustup toolchain install 1.99.1 --profile minimal --component clippy,rustfmt
rustup toolchain install nightly-2026-11-06 --profile minimal --component rustfmt
```

Install the pinned Cargo, formatting, link, spelling, and website tools from the repository root.
The platform scripts read their versions from `.devcontainer/.env`:

```sh
# Linux
bash scripts/install-tools.sh
```

```powershell
# Windows PowerShell
pwsh -NoProfile -File scripts/install-tools.ps1
```

On macOS, use the pinned manual Cargo-tool installation in [CONTRIBUTING.md](CONTRIBUTING.md) and
install Zola from its [official guide](https://www.getzola.org/documentation/getting-started/installation/).
The full `cargo make ci` task also needs `cargo-nextest`, `cargo-deny`, dprint, lychee, typos, and
Zola. `cargo make ci-rust` runs the Rust and dependency checks without the website build.
`cargo-machete`, `cargo-llvm-cov`, and `zizmor` support optional dependency, coverage, and workflow
checks.

To use the supplied devcontainer instead, install Docker with Compose and VS Code, then choose
**Reopen in Container**. The container installs the pinned development toolchain and tools for you.
See [CONTRIBUTING.md](CONTRIBUTING.md) for platform-specific details and the complete tool list.

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

The sample CLI writes structured tracing events to stderr and daily rotating log files. Set
`RUST_LOG` to change verbosity or `APP_LOG_DIR` to choose the log directory; see the
[logging guide](docs/guides/logging.md).

The sample library bounds TOML configuration reads to 1 MiB and saves configuration through a
staged atomic file replacement. See the [configuration reference](docs/development/configuration.md)
for the limits and file-replacement behavior.

Cargo writes build output to the checkout-relative `target/` directory by default. Set
`CARGO_TARGET_DIR` to use an external build directory, `SCCACHE_DIR` to relocate the sccache
cache, or `SCCACHE_BASEDIRS` to share cache entries across checkout paths. The shared-cache
worktree slots are `.worktrees/a`, `.worktrees/b`, and `.worktrees/c`.

On Windows, Cargo Make defaults `CARGO_TARGET_DIR` to `D:/target/<project>/<slot>` and
`CARGO_BUILD_BUILD_DIR` to `C:/code_build/<project>/<slot>`. The primary checkout uses the `main`
slot by default; set `CODE_WORKTREE_SLOT` to `a`, `b`, or `c` in a linked worktree. Set
`CARGO_TARGET_ROOT` or
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

Start at [`docs/README.md`](docs/README.md). The [requirements guide](docs/requirements.md)
includes a fictional todo CLI example; replace its behavior and quality targets when adopting the
template. See the [configuration reference](docs/development/configuration.md) for the purpose of
each repository configuration file. Add architecture notes, development instructions, and task
guides in the corresponding section.

## Contributing and license

Read [CONTRIBUTING.md](CONTRIBUTING.md) for setup, checks, and template adoption.
The root includes the contributor guide and dual-license texts. Crate READMEs link to the shared contributor guide and license; package-level license-file inclusion should be checked before publishing a crate.
Licensed under MIT OR Apache-2.0; see [LICENSE](LICENSE), [LICENSE-MIT](LICENSE-MIT), and
[LICENSE-APACHE](LICENSE-APACHE).

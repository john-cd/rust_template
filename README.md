<!-- TODO Reviewed ??-??-???? -->

# Rust workspace starter

A Rust workspace starter with a sample library, a Clap-based binary, developer tools, shared Cargo policy, a Docker Compose devcontainer, cross-platform CI, and a Zola website.

Create a project with `cargo-generate` 0.24 or newer:

```sh
cargo generate --git https://github.com/john-cd/rust_template.git --branch develop --name myproject
```

Replace `myproject` with your project name. The template currently uses `develop` as its integration branch.

The setup-version prompts default to this template's pins. If you change one, update the matching
toolchain or development-tool configuration in the generated project as well.

By default, the generated workspace names its CLI package after the selected project and
appends `_lib` to form the library package. `cargo-generate` also prompts for the crate folders and
package names; accept `auto` for package names to keep these defaults. Follow the
[adoption guide](docs/guides/adopting-template.md) to replace the sample behavior and metadata.

## Branch workflow

Use `develop` as the integration branch. Integrate reviewed work there, then promote a release to `main` when it is ready. Create feature branches from `develop`.

## Workspace layout

```text
.
├── crates/
│   ├── {{lib_folder_name}}/ # Sample library crate
│   └── {{cli_folder_name}}/ # Sample Clap binary
├── docs/                 # Project documentation
├── scripts/              # Repository maintenance and development scripts
├── tools/                # Stub for project-specific development and compile-time Rust tools
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

The library crate includes starter `config`, `error`, `models`, and `services` modules. The companion CLI uses `clap` derive and calls the library greeting: try `cargo run -p {{cli_package_name}} -- Alice`.

## Prerequisites

For local development,

1. Install Git, [rustup](https://rust-lang.org/tools/install/). On Windows, install the MSVC C++ build tools if `rustup` prompts for them.

2. The checked-in toolchain files pin Rust {{rust_version}} for development and `rustfmt` {{rustfmt_version}} for formatting;
   the declared MSRV is 1.98.1 and CI checks it separately.

Install both Rust toolchains with the minimal profile and the required components:

```sh
rustup toolchain install {{rust_version}} --profile minimal --component clippy,rustfmt
rustup toolchain install {{rustfmt_version}} --profile minimal --component rustfmt
```

Install the pinned `cargo` formatting, link, spelling, and website tools from the repository root.
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
cargo make f         # Apply Rust and repository formatting
cargo make fc        # Check Rust and repository formatting
cargo make fmt       # Check Rust formatting
cargo make check     # Check all workspace targets
cargo make c         # Short alias for check
cargo make build     # Build all workspace targets
cargo make b         # Short alias for build
cargo make clippy    # Run Clippy with warnings denied
cargo make l         # Short alias for clippy
cargo make fix       # Apply compiler suggestions
cargo make lint-fix  # Apply Clippy suggestions
cargo make run       # Run the sample CLI (alias: r)
cargo make release   # Build the release profile
cargo make test      # Run tests with nextest (alias: t)
cargo make bench     # Run benchmarks (bench-check compiles only)
cargo make doctest   # Run documentation tests
cargo make deny      # Check dependency policy
cargo make machete   # Review potentially unused dependencies
cargo make coverage  # Measure test coverage
cargo make coverage-html # Generate an HTML coverage report
cargo make zizmor    # Audit GitHub Actions workflows
cargo make pre       # Pre-commit formatting, check, and Clippy
cargo make doc       # Build API documentation
cargo make web       # Build the website
cargo make hooks-install # Install prek Git hooks
cargo make hooks-run # Run prek against staged files
cargo make build-env # Show Cargo output and cache environment
cargo make setup-build # Link direct Cargo output paths on Windows
cargo make docker-build-dev  # Build the development image
cargo make docker-build-prod # Build the production image
cargo make ci        # Run the complete local CI task set
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

On Windows, run `cargo make setup-build` once per checkout before using plain Cargo or
rust-analyzer. It creates or validates `target` and `build` junctions to the per-slot directories
below. Cargo Make tasks pass the same absolute paths directly. Set `CARGO_TARGET_ROOT` or
`CARGO_BUILD_ROOT` to change the roots; explicit `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`
values take precedence. Set `SCCACHE_DIR` to relocate the sccache cache, or `SCCACHE_BASEDIRS` to
share cache entries across checkout paths.

On Windows, Cargo Make stores target output in `D:/target/rust_template/<slot>` and intermediate
build output in `C:/code_build/rust_template/<slot>`. It detects `main` in the primary checkout and
`a`, `b`, or `c` in the matching `.worktrees/<slot>` checkout. Set `CODE_WORKTREE_SLOT` to override
the detected slot. Set `CARGO_TARGET_ROOT` or `CARGO_BUILD_ROOT` to change the respective roots;
explicit `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` values take precedence. Other platforms
keep Cargo's normal defaults.

## Devcontainer

To build the deployment image from a POSIX shell, load the shared image and tool versions:

```sh
set -a
. .devcontainer/.env
set +a
docker build -f .devcontainer/Dockerfile --target production \
  --build-arg RUST_VERSION --build-arg DEBIAN_VERSION --build-arg CARGO_BINSTALL_VERSION \
  --build-arg CARGO_CHEF_VERSION \
  --build-arg CARGO_PACKAGE_NAME={{cli_package_name}} -t {{project-name}} .
docker run --rm {{project-name}} Alice
```

The Cargo package names and production executable use the generated project name.

Open the repository in VS Code and choose **Reopen in Container**. The devcontainer uses the standalone `.devcontainer/compose.devcontainer.yaml` configuration to build from `.devcontainer/Dockerfile` on Debian Trixie, mount the workspace, and install the Rust Analyzer and TOML extensions. Edit `.devcontainer/.env` to configure its project prefix, production executable name, Rust version, and development-tool versions. It also installs the pinned Rust formatter and development tools. Port 1111 is forwarded for Zola previews.

The production service has a separate Compose file. Build it with `docker compose --env-file .devcontainer/.env -f .devcontainer/compose.production.yaml build app`, then run it with `docker compose --env-file .devcontainer/.env -f .devcontainer/compose.production.yaml run --rm app --help`.

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

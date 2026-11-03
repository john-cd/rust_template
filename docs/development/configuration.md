<!-- TODO Reviewed ??-??-???? -->

# Configuration reference

This page describes the checked-in files that control the workspace, tools, CI, and local development environment.

## Cargo workspace and dependencies

- [`Cargo.toml`](../../Cargo.toml) defines workspace members, shared package metadata, shared dependencies, and workspace-wide Rust and Clippy lints. Crate manifests inherit these values.
- [`Cargo.lock`](../../Cargo.lock) records the resolved versions for reproducible workspace builds. Update it with dependency changes; do not edit it by hand.
- [`.cargo/config.toml`](../../.cargo/config.toml) configures `sccache` as the compiler wrapper, sets its cache size, enables quiet Cargo output, and sets the network retry count. Cargo's default target directory applies unless overridden by `CARGO_TARGET_DIR`; the sample `.sccache` paths are commented examples. `dev` and `test` tune local builds; `ci` is the fast, repeatable profile used by CI; `debugging` retains full debug information; `tools` optimizes developer utilities. Install the pinned development tools from [CONTRIBUTING.md](../../CONTRIBUTING.md) before building.
- [`.config/nextest.toml`](../../.config/nextest.toml) defines default and CI test-runner behavior, including retries, timeouts, output, and the CI JUnit report.
- [`clippy.toml`](../../clippy.toml) holds Clippy thresholds and configuration shared by workspace packages.
- [`deny.toml`](../../deny.toml) defines dependency advisory, license, duplicate-version, and source policies for cargo-deny.
- [`rust-toolchain.toml`](../../rust-toolchain.toml) pins the Rust toolchain used for builds and includes Clippy and rustfmt components.
- [`rustfmt-toolchain.toml`](../../rustfmt-toolchain.toml) pins the formatter-only nightly needed by the shared unstable rustfmt options; build and CI compilation remain on the stable toolchain.
- [`rustfmt.toml`](../../rustfmt.toml) defines Rust formatting, import grouping, line widths, comment formatting, and Unix line endings.

## Development and editor settings

- [`prek.toml`](../../prek.toml) defines optional local Git hooks for file hygiene and formatter checks. Run `prek install` explicitly to enable them in a clone.
- [`.vscode/settings.json`](../../.vscode/settings.json), [extensions](../../.vscode/extensions.json), and [tasks](../../.vscode/tasks.json) provide portable editor defaults and Cargo Make commands. They contain no machine-specific build paths.

- [`Makefile.toml`](../../Makefile.toml) defines the `cargo make` tasks for formatting, checks, builds, Clippy, tests, doctests, dependency policy, and their combined CI task.
- [dprint.jsonc](../../dprint.jsonc) configures formatting for Markdown, JSON/JSONC, TOML, and YAML; use dprint fmt to format these files and dprint check to verify them.
- [`.editorconfig`](../../.editorconfig) sets text encoding, line endings, indentation, and trailing whitespace behavior for Rust, TOML, YAML, JSON, Markdown, and shell files.
- [`.gitattributes`](../../.gitattributes) defines Git's repository-wide text and line-ending normalization behavior.
- [`.gitignore`](../../.gitignore) excludes Rust build output, local environment files, and operating-system/editor state while allowing shared VS Code settings, tasks, launch configurations, and recommendations.
- [`.devcontainer/Dockerfile`](../../.devcontainer/Dockerfile) builds a Rust development image and creates the non-root `vscode` user.
- [`.devcontainer/compose.yaml`](../../.devcontainer/compose.yaml) builds the devcontainer service, mounts the checkout, and keeps the container available for the editor.
- [`.devcontainer/devcontainer.json`](../../.devcontainer/devcontainer.json) connects VS Code to the Compose service and recommends Rust Analyzer and TOML support.

## Automation

- [Issue forms](../../.github/ISSUE_TEMPLATE/bug_report.yml) and the [pull request template](../../.github/pull_request_template.md) collect reproduction steps, environment details, and validation evidence.

- [`.dockerignore`](../../.dockerignore) trims the devcontainer build context; [`.aiexclude`](../../.aiexclude) filters generated and local files from compatible AI indexing tools.
- [`lychee.toml`](../../lychee.toml) configures documentation link checking. `cargo make links` checks local links; `links-online` also checks remote URLs. Zola validates its own content references.
- [`tools/Cargo.toml`](../../tools/Cargo.toml) defines the workspace development binary, available through the `cargo tools` alias in Cargo configuration.
- [`website/config.toml`](../../website/config.toml) sets the Zola site title, local base URL, and link-checking policy. Change the URL before deploying.

- [`.github/dependabot.yml`](../../.github/dependabot.yml) schedules grouped monthly updates for Cargo dependencies, the Rust toolchain, GitHub Actions, and the devcontainer image and Compose setup.
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) runs formatting, workspace build checks, Clippy, tests, doctests, and dependency-policy checks on Linux, Windows, and macOS.

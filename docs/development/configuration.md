<!-- TODO Reviewed ??-??-???? -->

# Configuration reference

This page describes the checked-in files that control the workspace, tools, CI, and local development environment.

## Cargo workspace and dependencies

- [`Cargo.toml`](../../Cargo.toml) defines workspace members, shared package metadata, shared dependencies, and workspace-wide Rust and Clippy lints. Crate manifests inherit these values.
- [`Cargo.lock`](../../Cargo.lock) records the resolved versions for reproducible workspace builds. Update it with dependency changes; do not edit it by hand.
- [`cargo-generate.toml`](../../cargo-generate.toml) limits Liquid expansion to the generated project note and excludes local worktrees and build output from generated projects.
- [`.cargo/config.toml`](../../.cargo/config.toml) configures `sccache` as the compiler wrapper, quiet Cargo output, the `cargo tools` alias, and development, CI, debug, and tools profiles. Cargo uses the checkout-relative `target/` directory by default. Set `CARGO_TARGET_DIR` to move direct Cargo build output outside the checkout; set `SCCACHE_DIR` and `SCCACHE_BASEDIRS` in the environment to configure external/shared compiler-cache storage. For Cargo Make on Windows, `CARGO_TARGET_DIR` defaults to `D:/target/<project>` and `CARGO_BUILD_BUILD_DIR` to `C:/code_build/<project>`. Set `CARGO_TARGET_ROOT` or `CARGO_BUILD_ROOT` to change those roots; explicit output-directory variables take precedence. The shared-cache worktree slots are `.worktrees/a`, `.worktrees/b`, and `.worktrees/c`. `dev` tunes local builds; `ci` uses the same quick-compile settings; `debug` enables full debug information for workspace crates; `tools` optimizes the repository-local developer binary. Install the pinned development tools from [CONTRIBUTING.md](../../CONTRIBUTING.md) before building.
- [`.config/nextest.toml`](../../.config/nextest.toml) defines default and CI test-runner behavior, including retries, timeouts, output, and the CI JUnit report.
- [`clippy.toml`](../../clippy.toml) holds Clippy thresholds and configuration shared by workspace packages.
- [`deny.toml`](../../deny.toml) defines dependency advisory, license, duplicate-version, and source policies for cargo-deny.
- [`rust-toolchain.toml`](../../rust-toolchain.toml) pins the Rust toolchain used for builds and includes Clippy and rustfmt components.
- [`rustfmt-toolchain.toml`](../../rustfmt-toolchain.toml) pins the formatter-only nightly needed by the shared unstable rustfmt options; build and CI compilation remain on the stable toolchain.
- [`rustfmt.toml`](../../rustfmt.toml) defines Rust formatting, import grouping, line widths, comment formatting, and Unix line endings.

## Development and editor settings

- [`prek.toml`](../../prek.toml) defines optional local Git hooks for file hygiene and formatter checks. Run `prek install` explicitly to enable them in a clone.
- [`.vscode/settings.json`](../../.vscode/settings.json), [extensions](../../.vscode/extensions.json), and [tasks](../../.vscode/tasks.json) provide portable editor defaults and Cargo Make commands. They contain no machine-specific build paths.

  The extension recommendations include Bookmarks, Dependi, gitignore, Google Search, and markdownlint. CodeLLDB and Mermaid are optional recommendations. The devcontainer installs the core extensions; install the optional ones when debugging or editing diagrams.

- [`Makefile.toml`](../../Makefile.toml) owns the `cargo make` task definitions for formatting, checks, builds, Clippy, tests, doctests, dependency policy, and their combined CI task. The `cargo tools` alias is a convenience frontend whose format/check commands delegate to Cargo Make and whose website command invokes Zola.
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
- [`tools/Cargo.toml`](../../tools/Cargo.toml) defines the starter workspace development binary and is the home for project-specific development-only or compile-time utilities; the binary is available through the `cargo tools` alias, while `Makefile.toml` owns the underlying Cargo Make task definitions.
- [`website/config.toml`](../../website/config.toml) sets the Zola site title, local base URL, and link-checking policy. Change the URL before deploying.

- [`.github/dependabot.yml`](../../.github/dependabot.yml) schedules grouped monthly updates for Cargo dependencies, the Rust toolchain, GitHub Actions, and the devcontainer image and Compose setup.
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) runs formatting, workspace build checks, Clippy, tests, doctests, and dependency-policy checks on Linux, Windows, and macOS.

# Adopt this template

## Name and scope

Generate a new workspace with `cargo-generate` 0.24 or newer:

```sh
cargo generate --git https://github.com/john-cd/rust_template.git --branch develop --name my-project
```

For local development, use `cargo generate --path . --name my-project` from this
repository. The generated `PROJECT.md` records the chosen project name, package names, and source
folders. Accept `auto` for the CLI and library package prompts to derive the usual names from the
project name. Folder names default to `my_cli` and `my_lib` and can be changed independently.

For a non-interactive setup, provide the names directly:

```sh
cargo generate --path . --name todo-app \
  --define cli_folder_name=todo_cli \
  --define lib_folder_name=todo_core \
  --define cli_package_name=todo-cli \
  --define lib_package_name=todo-core
```

Package names accept lowercase letters, digits, hyphens, and underscores; generated Rust crate
identifiers convert hyphens to underscores.

1. Replace the shared version, keywords, categories, and other package metadata
   with values appropriate to the new project. Set package descriptions, authorship,
   repository URLs, and license notices. Keep `publish = false` until publication
   is intentional.
2. Replace the sample greeting and the unused configuration/model/service stubs
   with application behavior. Keep crate roots declarative.
3. Replace the illustrative todo CLI requirements and the website content with the new
   application's scope and quality targets. Set the website's deployment URL and check a build
   with its real URL prefix before publishing.
4. Review the branch workflow, issue forms, pull request template, dependency
   policy, test timeouts, MSRV, and tool pins. Enable the repository's Actions and
   dependency updates. Configure branch protection and security reporting in the
   hosting service; those settings are not created by these files.

The source folders default to `crates/my_lib` and `crates/my_cli`; choose different folder names
when cargo-generate prompts if that better fits the project. Package names are independent of those
paths. Use `rg -n 'rust_template|Rust workspace starter'`
from the root to find remaining template branding. Keep `Cargo.lock` synchronized
with manifest edits.

## Local setup

Install the tools listed in the contributor guide. Rustup reads the build pin
automatically. The nightly formatter is separate; install the channel from
`rustfmt-toolchain.toml` with the `rustfmt` component.

Cargo selects sccache through `.cargo/config.toml`. Direct Cargo commands use the checkout's
`target/` and `build/` paths; Cargo Make applies the repository's configured output paths. On
Windows, set `CODE_TARGET_ROOT` and `CODE_BUILD_ROOT` to choose external roots; the defaults are
`D:/target` and `D:/code_build`, with separate `<repo>/<slot>` directories. Run
`cargo make setup-build` once per checkout to connect plain Cargo and rust-analyzer to those
external slots. Set `CODE_SCCACHE_ROOT` or `SCCACHE_DIR` for the compiler cache. Cargo Make
sets `SCCACHE_BASEDIRS` to this repository's main checkout and registered worktrees; direct Cargo
uses the current checkout root. The shared worktree slots are `.worktrees/a`, `.worktrees/b`, and
`.worktrees/c`. To bootstrap without sccache, temporarily set `RUSTC_WRAPPER` to an empty value,
then install the cache wrapper or remove the project setting. In PowerShell use
`$env:RUSTC_WRAPPER = ''`; in a POSIX shell use `RUSTC_WRAPPER='' cargo check --workspace --locked`.

Optional hooks use the same checks as Cargo Make: install `prek` 0.5.6 or newer,
run `prek install`, then `prek run --all-files`. Hooks do not replace CI. Rust
Analyzer's format-on-save uses the build toolchain; the authoritative formatter
check remains `cargo make format-rust-check` with the pinned nightly.

## Validation and release

`Makefile.toml` owns the Cargo Make task definitions; `cargo tools format` and
`cargo tools check` are convenience commands that delegate to Cargo Make. The
`cargo tools website` command invokes Zola. CI separates Rust validation
(`cargo make ci-rust`) from repository formatting, links, and website validation.
The minimum-version check covers library/CLI targets including examples and
benchmarks, while repository tooling uses the pinned development compiler.

Use `cargo make test-cargo` when nextest is unavailable. Set `CARGO_TARGET_DIR` to
an appropriate local output directory when disk space requires it; never commit
machine-specific build or cache paths. `RUSTFMT_TOOLCHAIN` can override the local
formatter for experiments; CI checks the default pins for consistency.

Before a release, review public APIs and CLI compatibility, run the required checks
on supported operating systems, inspect each package with `cargo package --list`,
and verify license/readme inclusion. Add signing, artifact distribution, and
deployment only when the adopting project's targets and credentials are defined.
No publishing or deployment action runs automatically in this template.

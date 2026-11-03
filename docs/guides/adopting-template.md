# Adopt this template

## Name and scope

1. Rename `my_lib` and `my_cli` in their manifests, the workspace dependency,
   imports, tests, examples, benchmarks, documentation, and CLI output.
2. Set package descriptions, authorship, repository URLs, and license notices for
   the adopting project. Keep `publish = false` until publication is intentional.
3. Replace the sample greeting and the unused configuration/model/service stubs
   with application behavior. Keep crate roots declarative.
4. Replace the requirements and website content. Set the website's deployment URL
   and check a build with its real URL prefix before publishing.
5. Review the branch workflow, issue forms, pull request template, dependency
   policy, test timeouts, MSRV, and tool pins. Enable the repository's Actions and
   dependency updates. Configure branch protection and security reporting in the
   hosting service; those settings are not created by these files.

Use `rg -n 'my_lib|my_cli|rust_template|Rust workspace starter'` from the root to
find remaining template names. Keep `Cargo.lock` synchronized with manifest edits.

## Local setup

Install the tools listed in the contributor guide. Rustup reads the build pin
automatically. The nightly formatter is separate; install the channel from
`rustfmt-toolchain.toml` with the `rustfmt` component.

Cargo currently selects sccache through `.cargo/config.toml`. To bootstrap without
it, temporarily set `RUSTC_WRAPPER` to an empty value, then install the cache wrapper
or remove the project setting. In PowerShell use `$env:RUSTC_WRAPPER = ''`; in a
POSIX shell use `RUSTC_WRAPPER='' cargo check --workspace --locked`.

Optional hooks use the same checks as Cargo Make: install `prek` 0.5.6 or newer,
run `prek install`, then `prek run --all-files`. Hooks do not replace CI. Rust
Analyzer's format-on-save uses the build toolchain; the authoritative formatter
check remains `cargo make fmt` with the pinned nightly.

## Validation and release

Run `cargo tools format` and `cargo tools check`. CI separates Rust validation
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

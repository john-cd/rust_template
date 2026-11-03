# Contributing

Create focused branches from `develop`. Keep changes small enough to review and describe the problem,
resulting behavior, and validation in your pull request. Releases are promoted to `main`.

## Setup

Use rustup to install the build toolchain from `rust-toolchain.toml` and the formatter toolchain from
`rustfmt-toolchain.toml`. The build pin and formatter pin also appear in CI and the devcontainer; keep
them synchronized when updating. The declared Rust 1.85 minimum must be checked separately before
claiming compatibility after dependency or language changes.

The shared devcontainer versions are in `.devcontainer/.env`. Edit and commit version changes there;
these values are not secrets. Compose requires these values and passes them to the Docker build.
Store any credentials in a separate ignored secret file. The devcontainer uses the Debian Trixie Rust
image. After changing versions, rebuild the container. The toolchain pins used by Cargo and CI remain in
`rust-toolchain.toml`, `rustfmt-toolchain.toml`, and the workflow; update those too when changing the
workspace's compiler or formatter version.

Install `cargo-binstall` first with `cargo install --locked cargo-binstall --version 1.26.2`, then install the development
tools with its prebuilt-binary support (CI and the devcontainer also install these):

```sh
cargo binstall --locked --no-confirm cargo-make@0.37.24 cargo-nextest@0.9.148 cargo-deny@0.20.2 dprint@0.64.3 lychee@0.24.2 sccache@0.19.0 zola@0.23.4
```

Install [Zola 0.23.4](https://www.getzola.org/documentation/getting-started/installation/) for the website.
On Windows, put developer tools under `C:/tools` and package caches under `D:/packages`.
The repository itself uses portable relative paths and requires no global compiler cache or linker.

## Validation

Optional Git hooks use [prek.toml](prek.toml). Install `prek` 0.5.6 or newer, then run
`prek install` and `prek run --all-files`. Hook installation is local to your clone.
The hooks check file hygiene and call the existing formatter checks; CI remains
the full validation gate. See the [adoption guide](docs/guides/adopting-template.md)
for fresh-checkout setup, compiler-cache bootstrap, and release decisions.

```sh
cargo tools format       # Apply Rust and dprint formatting
cargo make --quiet pre    # Formatting, check, and Clippy
cargo tools check        # Complete local CI, including dependency and documentation checks
cargo make --quiet links-online # Optional external-link check (requires network access)
cargo run -p my_cli -- Alice
```

Commit `Cargo.lock` changes together with manifest changes. Add meaningful tests for changed behavior.
Run doctests as well as nextest: nextest does not execute documentation examples. Keep workspace lint
inheritance on every crate; document any narrowly scoped exception. Never commit credentials or local
environment files. Ignore files are convenience filters, not a security boundary.

## Layout and documentation

Application and library crates live under `crates/`; the development binary lives under `tools/`.
Each crate has a README. The root contributor guide and dual-license texts apply to all workspace
crates. When adding a crate, inherit shared metadata and lints; inspect its package contents before
publishing.

Keep architecture, contributor instructions, and reference material under [docs](docs/README.md).
Edit public-facing content and templates under [website](website/README.md). Before using this starter
for another project, rename the crates, update authorship and license notices as appropriate, and set
the website's real `base_url`. The starter does not publish or deploy automatically.

Unless explicitly stated otherwise, contributions are licensed under the same MIT OR Apache-2.0
terms as the workspace. See [LICENSE](LICENSE).

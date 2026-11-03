+++
title = "Getting started"
description = "Build the workspace and make it your own."
weight = 1
+++

## Build and run

Install the Rust toolchain pinned in the repository, then run:

```sh
cargo check --workspace --all-targets --locked
cargo run -p my_cli -- Alice
```

## Make it yours

Rename the library and binary packages, update workspace metadata and license notices, and replace
this starter content. Add reusable code under `crates/`, development commands under `tools/`, and
architecture and contributor notes under `docs/`.

## Contributor checks

Follow the root `CONTRIBUTING.md` to install development tools. Run `cargo tools format` before
committing, then `cargo tools check` to validate Rust, dependencies, links, and this website.

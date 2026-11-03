# TODO

- [ ] Rename the project and update package metadata in `Cargo.toml`.
- [ ] Replace the template code with your application logic.
- [ ] Add or update tests, then run `cargo test`.
- [ ] Update the README with setup and usage instructions.
- [ ] Run `cargo fmt` and `cargo clippy`.

## Repository review follow-up: November 2, 2026

- [ ] **P2: Validate hosted, container, and website acceptance separately.** Run the
      three-OS workflow, non-root devcontainer build/start, and Zola build with the
      updated description metadata. Record the exact revision and artifacts. Local
      syntax and Rust tests do not establish those runtime results.

Read-only validation on the evolving working tree: workspace/all-target/all-feature
`cargo check` and Clippy with `--locked --profile ci` passed; workspace all-target
tests/examples/benchmark smoke and doctests passed. `dprint check` passed. Focused
offline lychee over the root/contributor, crate/tool READMEs, documentation index,
and configuration reference checked 54 links with zero errors (four exclusions).
Earlier undefined-error/ExitCode and CLI README-link findings are resolved in the
observed source; no source fixes were made by this review. This follow-up also passed
the locked Rust 1.85.0 library/CLI all-target check, the `debug` profile check,
dprint, and the pinned Rust formatting check. Hosted CI, Docker, and Zola runtime
acceptance remain unverified here.

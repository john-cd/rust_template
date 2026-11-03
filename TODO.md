# TODO

- [ ] Rename the project and update package metadata in `Cargo.toml`.
- [ ] Replace the template code with your application logic.
- [ ] Add or update tests, then run `cargo test`.
- [ ] Update the README with setup and usage instructions.
- [ ] Run `cargo fmt` and `cargo clippy`.

## Repository review follow-up: November 2, 2026

This repository has no GitHub remote; keep review tasks here. The checkout is being
edited concurrently. Review findings below describe the files observed during this
review and do not replace or commit that existing work.

- [ ] **P1: Install the compiler wrapper before the first CI Cargo check.**
      `.github/workflows/ci.yml` sets `RUSTC_WRAPPER=sccache` for the whole validation
      job, but the minimum-Rust-version Cargo check runs before the sccache install
      action. Install the pinned wrapper before any Cargo command that invokes rustc,
      or explicitly disable it for the earlier check. Acceptance: a clean runner with
      no preinstalled sccache reaches and executes the MSRV check successfully.
- [ ] **P2: Validate the declared Rust 1.85 support.** The new hosted step exists,
      but this review ran Rust 1.99.1 only. Run the locked library/CLI target scope on
      1.85 and record the result; distinguish a configured check from a passing check.
      Keep tools/benchmarks outside the support promise unless explicitly validated.
- [ ] **P2: Reconcile the debugging profile's dependency symbols.**
      `.cargo/config.toml` describes full debug information but
      `[profile.debugging.package."*"]` still sets `debug = 0`. Choose and document
      the intended dependency-debugging contract; verify effective compiler flags and
      preserve any deliberate cache-reuse tradeoff. Do not copy another repo's profile
      without checking this template's intended defaults.
- [ ] **P2: Validate hosted, container, and website acceptance separately.** Run the
      three-OS workflow, non-root devcontainer build/start, and Zola build with the
      updated description metadata. Record the exact revision and artifacts. Local
      syntax and Rust tests do not establish those runtime results.
- [ ] **P3: Align portable storage guidance with the other Rust repositories.**
      AGENTS.md says Cargo config owns target/build paths, while the local config
      leaves those paths unset and relies on host defaults. Document a portable
      fresh-checkout contract and configurable external roots, the owning command
      runner, supported worktree slots, and the distinction between template defaults
      and machine-specific host fallback. Preserve explicit permission before Git
      integration operations in this template's guidance.

Read-only validation on the evolving working tree: workspace/all-target/all-feature
`cargo check` and Clippy with `--locked --profile ci` passed; workspace all-target
tests/examples/benchmark smoke and doctests passed. `dprint check` passed. Focused
offline lychee over the root/contributor, crate/tool READMEs, documentation index,
and configuration reference checked 54 links with zero errors (four exclusions).
Earlier undefined-error/ExitCode and CLI README-link findings are resolved in the
observed source; no source fixes were made by this review. Hosted CI, MSRV, Docker,
and Zola runtime acceptance remain unverified here.

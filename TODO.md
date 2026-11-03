# TODO

- [ ] Rename the project and update package metadata in `Cargo.toml`.
- [ ] Replace the template code with your application logic.
- [ ] Add or update tests, then run `cargo test`.
- [ ] Update the README with setup and usage instructions.
- [ ] Run `cargo fmt` and `cargo clippy`.

## Repository review follow-up: November 2, 2026

- [x] **P1: Repair Cargo Make formatting and CI tasks.** After refreshing to
      `origin/develop` (`f0f8f50`), `pre` still referenced removed `fmt` and
      `format-check` task definitions, while `format-rust-check` ran rustfmt in
      write mode. The hook and adoption guide also invoked the nonexistent `fmt`
      task. Grouped task definitions by topic, placed each short alias next to its
      target, wired `format-check` to both read-only format checks, made
      `format-rust-check` use `--check`, and updated the hook and guide references.
- [ ] **P2: Validate hosted, container, and website acceptance separately.** Run the
      three-OS workflow, non-root devcontainer build/start, and Zola build with the
      updated description metadata. Record the exact revision and artifacts. Local
      syntax and Rust tests do not establish those runtime results.

Linux validation used refreshed revision `f0f8f50` plus the review edits:

- `cargo make format-check` and `cargo make ci-rust` passed with cargo-make 0.37.24:
  workspace check/build, Clippy with warnings denied, nextest (11 passed), doctests,
  rustdoc, and cargo-deny. Cargo deny reported unused license allowances in
  `deny.toml` as warnings.
- Pinned nightly rustfmt, `dprint check`, `git diff --check`, and TOML parsing passed.
  Static inspection found all 47 Cargo Make task aliases and dependencies resolve;
  the formatting task runs both Rust and repository checks in read-only mode.
- Offline lychee checked 65 links with zero errors and 10 configured exclusions.
  Docker Compose configuration passed. The devcontainer image build could not reach
  `deb.debian.org` from Docker's build network; no container start was possible.
- Hosted three-OS CI remains unrun. Zola and zizmor are not installed in this Linux
  workspace, so website build/check and workflow security audit remain unverified.

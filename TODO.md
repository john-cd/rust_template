# TODO

- [ ] Rename the project and update package metadata in `Cargo.toml`.
- [ ] Replace the template code with your application logic.
- [ ] Add or update tests, then run `cargo test`.
- [ ] Update the README with setup and usage instructions.
- [ ] Run `cargo fmt` and `cargo clippy`.

## Repository review follow-up: November 2, 2026

- [x] **P2: Devcontainer build and non-root startup accepted (2026-11-04).** Docker
      Compose configuration and `docker compose build` passed at revision `9fd8935`;
      the container runs as `uid=1000(vscode)` with rustc 1.99.1 and Zola 0.23.6.

Linux validation against refreshed `origin/develop` and the review changes:

- `cargo make format-check`, `cargo make pre`, `cargo make ci-rust` (11 tests
  passed), and `prek run --all-files` passed. Cargo deny emitted warnings for unused
  license allowances in `deny.toml`.
- Pinned nightly rustfmt, dprint, TOML parsing, and `git diff --check` passed. The
  Makefile has 47 tasks; aliases and dependencies resolve, and format checks run in
  read-only mode. Offline lychee checked 65 links with zero errors and 10 exclusions.
- Installing Zola from its pinned 0.23.6 upstream release now verifies the SHA-256
  before installing. The Linux install test, Zola website check/build, and Compose
  configuration pass.
- PR #2's Rust matrix passed at head `8ad5203`. The documentation job exposed setup
  gaps: Zola is a release binary, not a versioned Cargo package, and the pinned nightly
  formatter must be exported and installed in that job. Both paths are fixed; the full
  documentation job passed at head `c10f055`.

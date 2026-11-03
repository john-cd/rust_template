# TODO

- [ ] Rename the project and update package metadata in `Cargo.toml`.
- [ ] Replace the template code with your application logic.
- [ ] Add or update tests, then run `cargo test`.
- [ ] Update the README with setup and usage instructions.
- [ ] Run `cargo fmt` and `cargo clippy`.

## Repository review follow-up: November 2, 2026

- [ ] **P2: Complete devcontainer build and non-root startup acceptance.** Compose
      configuration passes, but Docker's build network could not resolve
      `deb.debian.org`; retry on a host with package-mirror access and record the
      container identity and revision.

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
- PR #2's Rust matrix passed at head `8ad5203`. The documentation job exposed two
  setup gaps on later heads: Zola is a release binary, not a versioned Cargo package,
  and the job must install the pinned nightly formatter. Both paths are fixed; verify
  the documentation job on the next PR run.

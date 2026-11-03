# Repository Instructions

## Workspace structure

- The root `Cargo.toml` owns shared dependency versions, package metadata, and lint policy. Crates
  inherit shared dependencies and lints from the workspace.
- `crates/my_lib` owns reusable library behavior; `crates/my_cli` owns command-line parsing and
  process-level behavior. Treat `tools/` as a starter home for project-specific development-only
  and compile-time utilities; keep standalone maintenance scripts in `scripts/`.
- Keep crate roots such as `lib.rs` declarative. Place implementation in child modules grouped by
  responsibility; use `modules/` when a cohesive module tree needs multiple child modules.
- Preserve the library's public paths when reorganizing modules. Re-export public modules or items
  from `lib.rs` rather than adding logic there.

## Rust changes

- Keep module responsibilities cohesive and use concrete types and functions. Add abstractions only
  when they give callers a useful boundary or enable real variation.
- Add doc comments to public APIs and keep crate-level and module-level documentation current.
- Put unit tests next to the implementation they exercise. Put cross-module or external-API tests
  under each crate's `tests/` directory. Keep runnable examples under `examples/` and Criterion
  benchmarks under `benches/`.
- Use workspace dependencies from the root manifest. Commit `Cargo.lock` updates with manifest
  changes.
- Inherit workspace lints. Do not weaken lint levels to silence a diagnostic without a narrow,
  documented reason.

## Commands and validation

- Cargo uses `.cargo/config.toml` for sccache, quiet output, the `cargo tools` alias using the `tools` profile, and development profiles. Cargo Make overrides Windows build output to `D:/target/<project>` and `C:/code_build/<project>` by default; `CARGO_TARGET_ROOT` and `CARGO_BUILD_ROOT` change those roots, while explicit `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` values take precedence. `SCCACHE_DIR` and `SCCACHE_BASEDIRS` can configure external compiler-cache storage. The shared cache recognizes the `.worktrees/a`, `.worktrees/b`, and `.worktrees/c` worktree slots.
- `Makefile.toml` owns Cargo Make task definitions. The `cargo tools` alias is a convenience
  frontend: its format/check commands delegate to Cargo Make, and its website command invokes Zola. Keep local commands and CI tasks consistent with those definitions.
- For focused validation, use `cargo check --workspace --all-targets --all-features --locked
  --profile ci`, `cargo test --workspace --all-targets --all-features --locked --profile ci`, and
  `cargo test --workspace --doc --all-features --locked --profile ci` as appropriate.
- Run formatting and the focused checks relevant to a change. Do not claim platform or CI validation
  that was not run.

## Editing and Git

- Preserve unrelated staged, unstaged, and untracked changes. Keep edits scoped to the request.
- Do not commit, amend, rebase, or push unless the user requests that Git operation.
- Keep documentation, examples, tests, and tool configuration synchronized with behavior changes.
- Preserve existing human-review markers when editing files that contain them.

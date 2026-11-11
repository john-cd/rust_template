# Project adoption checklist

Use this list after generating a project with `cargo-generate`. Keep the tasks that apply, delete the rest, and add
project-specific work. The template already substitutes the generated project name in selected
files; review the result before changing package names or source-folder paths.

## Decide what to build

- [ ] Replace the fictional todo CLI in [`docs/requirements.md`](docs/requirements.md) with the
      application's functional requirements, acceptance examples, and explicit out-of-scope items.
- [ ] Set measurable non-functional targets for applicable qualities: reliability, security,
      privacy, performance, availability, portability, maintainability, observability, and resource use.
- [ ] Decide the user-facing interfaces, supported operating systems and architectures, minimum
      Rust version, data formats, configuration sources, and deployment model.

## Adopt the repository identity

- [ ] Review `PROJECT.md`, `README.md`, crate READMEs, and website content; replace starter text with
      the project's purpose, setup, examples, and support information.
- [ ] Update workspace and crate metadata in `Cargo.toml`: descriptions, version, authors, URLs,
      keywords, categories, license, and `publish` policy. Check package names generated from the
      project name and change them only if the project needs different names.
- [ ] Search for remaining template names and sample paths with e.g. `ripgrep`:
      `rg -n 'rust_template|Rust workspace starter|my_cli|my_lib' --glob '!Cargo.lock' .` and update
      intentional references.
- [ ] Replace the example requirements and quality targets; keep the docs index and adoption guide
      current when adding or removing project documentation.
- [ ] Set the website deployment URL in `website/config.toml`; replace the website's sample content
      or remove the website if the project will not maintain one.
- [ ] Review issue forms, the pull request template, license notices, branch workflow, and
      `.github/dependabot.yml`. Enable Actions, dependency updates, branch protection, and security
      reporting in the hosting service as appropriate.

## Replace the starter application

- [ ] Replace the greeting in `crates/{{lib_folder_name}}/src/greeting.rs` and the sample command in
      `crates/{{cli_folder_name}}/src/cli.rs` with application behavior. Remove unused model, service, example, and
      benchmark stubs; add modules where responsibilities need a clear home.
- [ ] Define command names, options, validation, help examples, exit codes, and which results and
      diagnostics go to stdout and stderr. Document behavior when a downstream pipe closes.
- [ ] Replace the example `Config` fields and `MY_LIB_*` prefix. Set defaults, source precedence,
      validation, and the input-size limit (`MAX_CONFIG_FILE_BYTES`, 1 MiB by default) to match the
      application.
- [ ] Adapt `MyError` to the application's failure cases. Include useful operation and path context
      while preserving the cause; report each CLI failure once and keep secrets out of errors and logs.
- [ ] Decide whether the application needs persistent data. If it does, define its format, file
      ownership, backup and recovery behavior, and interrupted-write guarantees. Adapt the atomic file
      helper and test replacement failures, links, and permissions on supported platforms.
- [ ] Keep library crates independent of global subscriber setup. Add tracing events at useful
      boundaries, choose fields that do not expose secrets, and review the log directory, rotation,
      retention, queue, and filtering policy for the deployment. The sample retains up to seven daily
      files by count; apply a size limit when disk quotas require one.

## Manage dependencies and workspace policy

- [ ] Add shared dependency versions and features to `[workspace.dependencies]` in `Cargo.toml`,
      then inherit them from each crate with `dependency.workspace = true`. Keep optional features
      narrow and remove dependencies no longer used.
- [ ] After manifest changes, run `cargo check --workspace --all-targets` to resolve dependencies
      and update `Cargo.lock`; review and keep the lockfile change. Then use locked checks. `cargo make
      update` refreshes the compatible dependency graph more broadly, so review its full lockfile diff.
- [ ] Run `cargo make deny` for advisory, license, duplicate-version, and source policy. Run
      `cargo make machete` to review unused dependencies.
- [ ] Change `rust-version`, the pinned toolchains, MSRV CI, and tool-version pins together when the
      project changes its compiler or development-tool requirements.
- [ ] Add runtimes, concurrency frameworks, or specialized containers such as `SmallVec` only when
      the application needs them; set workload limits and measure before adding optimization-specific
      dependencies.

## Tests and project documentation

- [ ] Add unit tests for domain rules and configuration precedence; integration and CLI tests for
      arguments, exit status, stdout/stderr, filesystem effects, and failure cases.
- [ ] Keep tests deterministic and independent of live services where possible. Add fixtures or
      test doubles for external systems; add benchmarks only for a defined performance question.
- [ ] Update examples, API documentation, operator guidance, and runbooks when behavior changes.
      Keep README commands and dependency instructions aligned with `Makefile.toml` and CI.
- [ ] Review the CI platform matrix, test timeouts, permissions, workflow pins, container smoke
      checks, and dependency-update rules for the project's actual targets.

## Validate before integrating or releasing

Run the checks that cover the changes. The repository defines these `cargo make` tasks:

```sh
cargo make format        # Apply Rust, Markdown, and configuration formatting
cargo make format-check  # Check formatting without modifying files
cargo make check         # Check all workspace targets and features
cargo make lint-check    # Run Clippy with warnings denied
cargo make test          # Run tests with cargo-nextest
cargo make test-cargo    # Use Cargo's test runner when nextest is unavailable
cargo make doctest       # Run documentation tests
cargo make deny          # Check dependency policy
cargo make machete       # Review unused dependencies
cargo make typos         # Check spelling
cargo make links         # Check local documentation links
cargo make ci-rust       # Run the Rust, test, docs, and dependency checks
cargo make ci            # Run the complete local CI task set
```

- [ ] Run formatting, build, lint, and test checks for the changed crates; run `cargo make ci` when
      the required local tools are installed.
- [ ] After website changes, run `cargo make website-check` and `cargo make website-build`. After
      container changes, run `cargo make docker-build-dev` and/or `cargo make docker-build-prod`.
- [ ] For publishable crates, inspect `cargo package -p <crate> --list`, verify license and README
      inclusion, and review the package contents before publishing.
- [ ] Review the final diff and generated files. Do not commit build output, local cache paths,
      credentials, or machine-specific configuration.

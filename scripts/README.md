# Scripts

Place repository maintenance and development scripts in this directory. Prefer small scripts that
reuse the Cargo tasks and tool versions documented in the repository instead of duplicating their
configuration.

## Install local tools on Windows

Run `pwsh -NoProfile -File scripts/install-tools.ps1` from any directory. The script reads pinned
versions from `.devcontainer/.env`, installs Cargo tools into Cargo's configured default install
root, and adds that root's `bin` directory to the current user's `PATH` when needed. It installs
GitHub CLI through WinGet. Use `-DryRun` to print the planned commands without installing tools or
changing `PATH`.
Both installers also add cargo-machete, cargo-llvm-cov, zizmor, and Rust's `llvm-tools-preview`
component for coverage.

The script requires Rust/Cargo and WinGet. Open a new terminal after installation so updated user
`PATH` entries are loaded.

## Install local tools on Linux

From the repository root, run `bash scripts/install-tools.sh`. The script reads the same pinned versions
from `.devcontainer/.env`, installs Cargo tools with `cargo-binstall`, verifies and installs the
pinned Zola release binary, and installs GitHub CLI with
the available apt, dnf, pacman, or zypper package manager. Rust/Cargo is required; GitHub CLI may
require `sudo`. Use `--dry-run` to print commands without installing anything, or `--skip-gh` to
manage GitHub CLI separately. Ensure Cargo's install-root `bin` directory is on `PATH` afterward.

## Cloud environment setup script (Linux)

`scripts/setup-cloud.sh` is a self-contained, idempotent installer for fresh Linux containers such
as claude.ai cloud environments. Paste its content into the environment's **Setup script** field
(it must not depend on a checkout, because setup runs before repositories are cloned), or run
`bash scripts/setup-cloud.sh` in a clone. It prefers prebuilt binaries (`cargo-binstall`, release
tarballs) over compiling, verifies Zola with a pinned SHA-256, and keeps going after a single tool
fails, then exits non-zero with a summary.

```sh
bash scripts/setup-cloud.sh                       # default: rust, cargo, fmt, python, just
bash scripts/setup-cloud.sh --with docs,web       # add groups
bash scripts/setup-cloud.sh --only rust,cargo     # exact selection
bash scripts/setup-cloud.sh --with all --skip gh  # everything except GitHub CLI
bash scripts/setup-cloud.sh --dry-run --with all  # print commands only
SETUP_COMPONENTS=default,docs bash scripts/setup-cloud.sh   # same via environment
```

Groups: `apt` (build-essential, clang, lld, mold, cmake, pkg-config, OpenSSL/SQLite headers,
protoc), `rust` (rustup, clippy, rustfmt, llvm-tools, pinned nightly rustfmt), `cargo` (cargo-make,
nextest, deny, machete, llvm-cov, sccache), `fmt` (dprint, lychee, typos, zizmor, prek,
shellcheck), `python` (latest uv), `just`, `extras` (audit, sort, outdated, udeps, hack, insta,
fuzz, chef, generate, watch), `web` (wasm32 target, trunk, wasm-bindgen-cli, dioxus-cli,
binaryen), `docs` (Zola, mdBook, admonish, linkcheck), `gh` (GitHub CLI, actionlint).

Versions: the pins at the top of the script mirror `.devcontainer/.env`; when `./.devcontainer/.env`
exists (or `ENV_FILE` points to a file) it overrides them, and any `<TOOL>_VERSION` environment
variable overrides both (for example `CARGO_NEXTEST_VERSION=0.9.150`). uv, just, prek, mdBook and
the extras use the latest release unless pinned the same way. Update the script's pins and Zola
checksums together with `.devcontainer/.env`.

Cargo's `bin` directory and `~/.local/bin` are appended to `~/.bashrc` and `~/.profile`. Verify
locally with `bash -n`, `shellcheck`, and `--dry-run`; real installs need network access to
GitHub, crates.io, static.rust-lang.org and astral.sh.

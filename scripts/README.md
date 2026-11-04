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

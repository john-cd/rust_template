#!/usr/bin/env bash

# Install pinned Rust development tools and, unless skipped, the GitHub CLI.

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/install-tools.sh [--dry-run] [--skip-gh]

Install the pinned Cargo development tools and GitHub CLI on Linux.
Rust and Cargo must already be installed. --skip-gh omits the distro package.
EOF
}

dry_run=false
skip_gh=false
for arg in "$@"; do
  case "$arg" in
    --dry-run) dry_run=true ;;
    --skip-gh) skip_gh=true ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown option: %s\n' "$arg" >&2; usage >&2; exit 2 ;;
  esac
done

if [[ $(uname -s) != Linux ]]; then
  printf 'This installer requires Linux.\n' >&2
  exit 1
fi
if ! command -v cargo >/dev/null 2>&1; then
  printf 'Rust/Cargo is required. Install Rust with rustup, then rerun this script.\n' >&2
  exit 1
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd -- "$script_dir/.." && pwd)
# Keep versions centralized with the devcontainer image to prevent installer drift.
version_file="$repository_root/.devcontainer/.env"
if [[ ! -f $version_file ]]; then
  printf 'Could not find tool versions at %s.\n' "$version_file" >&2
  exit 1
fi

declare -A versions=()
while IFS= read -r line || [[ -n $line ]]; do
  line=${line%$'\r'}
  if [[ $line =~ ^([A-Z][A-Z0-9_]*)=([A-Za-z0-9._-]+)$ ]]; then
    versions[${BASH_REMATCH[1]}]=${BASH_REMATCH[2]}
  fi
done < "$version_file"

required_versions=(
  CARGO_BINSTALL_VERSION CARGO_MAKE_VERSION CARGO_NEXTEST_VERSION
  CARGO_DENY_VERSION CARGO_MACHETE_VERSION CARGO_LLVM_COV_VERSION
  ZIZMOR_VERSION SCCACHE_VERSION DPRINT_VERSION LYCHEE_VERSION
  ZOLA_VERSION TYPOS_VERSION
)
for name in "${required_versions[@]}"; do
  if [[ -z ${versions[$name]:-} ]]; then
    printf 'Missing %s in %s.\n' "$name" "$version_file" >&2
    exit 1
  fi
done

run() {
  printf '> '
  printf '%q ' "$@"
  printf '\n'
  if [[ $dry_run == false ]]; then
    "$@"
  fi
}

# Run from the checkout so Cargo workspace discovery and relative paths are stable.
cd -- "$repository_root"
run rustup component add llvm-tools-preview
run env RUSTC_WRAPPER= cargo install --locked cargo-binstall --version "${versions[CARGO_BINSTALL_VERSION]}"

if [[ $dry_run == true ]]; then
  binstall=cargo-binstall
else
  # Prefer PATH, then fall back to Cargo's configured install root.
  binstall=$(command -v cargo-binstall || true)
  if [[ -z $binstall ]]; then
    install_root=${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}
    binstall="$install_root/bin/cargo-binstall"
  fi
  if [[ ! -x $binstall ]]; then
    printf 'cargo-binstall was not found. Add its install root bin directory to PATH.\n' >&2
    exit 1
  fi
fi

run env RUSTC_WRAPPER= "$binstall" --locked --no-confirm \
  "cargo-make@${versions[CARGO_MAKE_VERSION]}" \
  "cargo-nextest@${versions[CARGO_NEXTEST_VERSION]}" \
  "cargo-deny@${versions[CARGO_DENY_VERSION]}" \
  "cargo-machete@${versions[CARGO_MACHETE_VERSION]}" \
  "cargo-llvm-cov@${versions[CARGO_LLVM_COV_VERSION]}" \
  "zizmor@${versions[ZIZMOR_VERSION]}" \
  "sccache@${versions[SCCACHE_VERSION]}" \
  "dprint@${versions[DPRINT_VERSION]}" \
  "lychee@${versions[LYCHEE_VERSION]}" \
  "typos-cli@${versions[TYPOS_VERSION]}"

install_root=${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}
# Zola shares the Cargo tool install root, but comes from a checksum-verified release archive.
run bash "$repository_root/scripts/install-zola.sh" "${versions[ZOLA_VERSION]}" "$install_root/bin"

# Install GitHub CLI with the host package manager because it is not a Cargo tool.
if [[ $skip_gh == true ]]; then
  printf 'Skipping GitHub CLI.\n'
elif command -v gh >/dev/null 2>&1; then
  printf 'GitHub CLI is already installed.\n'
else
  privilege=()
  if [[ $(id -u) -ne 0 ]]; then
    if ! command -v sudo >/dev/null 2>&1 && [[ $dry_run == false ]]; then
      printf 'sudo is required to install GitHub CLI with the system package manager.\n' >&2
      exit 1
    fi
    privilege=(sudo)
  fi

  if command -v apt-get >/dev/null 2>&1; then
    run "${privilege[@]}" apt-get update
    run "${privilege[@]}" apt-get install -y gh
  elif command -v dnf >/dev/null 2>&1; then
    run "${privilege[@]}" dnf install -y gh
  elif command -v pacman >/dev/null 2>&1; then
    run "${privilege[@]}" pacman -S --needed --noconfirm github-cli
  elif command -v zypper >/dev/null 2>&1; then
    run "${privilege[@]}" zypper --non-interactive install gh
  else
    printf 'No supported package manager found for GitHub CLI; install gh separately.\n' >&2
    exit 1
  fi
fi

printf 'Installed Cargo tools. Ensure %s/bin is on PATH.\n' "${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}"

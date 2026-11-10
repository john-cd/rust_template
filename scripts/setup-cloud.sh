#!/usr/bin/env bash
# TODO Reviewed ??-??-????
#
# Idempotent setup script for Linux cloud / CI-like environments (for example the
# claude.ai "cloud environment" setup script field). Self-contained: paste or copy it as is.
# Installs the tooling used across the repos with prebuilt binaries (cargo-binstall) first.
#
# Usage: setup-cloud.sh [options]
#   --only a,b,c     install only these groups (default: SETUP_COMPONENTS or "default")
#   --with a,b       add groups to the selection
#   --skip a,b       remove groups from the selection
#   --list           print groups and exit
#   --dry-run        print commands without running them
#   -h, --help       this help
# Environment: SETUP_COMPONENTS, SETUP_SKIP, SETUP_WITH (comma lists), SETUP_DRY_RUN=1,
#   <TOOL>_VERSION overrides (for example CARGO_NEXTEST_VERSION=0.9.150), ENV_FILE=path to a
#   KEY=VALUE file (default: ./.devcontainer/.env when present) that overrides the pins below.
#
# Groups (default = rust,cargo,fmt,python,just):
#   apt      system packages (build-essential clang lld mold cmake pkg-config libssl-dev ...)
#   rust     rustup if missing, stable components (clippy rustfmt llvm-tools-preview),
#            pinned nightly rustfmt toolchain
#   cargo    cargo-binstall, cargo-make, cargo-nextest, cargo-deny, cargo-machete,
#            cargo-llvm-cov, sccache
#   fmt      dprint, lychee, typos, zizmor, prek, shellcheck (apt)
#   python   uv (latest; installs pinned Python on demand via `uv python install`)
#   just     just
#   extras   cargo-audit, cargo-sort, cargo-outdated, cargo-udeps, cargo-hack, cargo-insta,
#            cargo-fuzz, cargo-chef, cargo-generate, cargo-watch
#   web      wasm32-unknown-unknown target, trunk, wasm-bindgen-cli, dioxus-cli, wasm-opt (apt)
#   docs     zola (checksum verified), mdbook, mdbook-admonish, mdbook-linkcheck
#   gh       GitHub CLI (apt/dnf), actionlint
#   all      every group above

set -euo pipefail

# ---- Pins: keep in sync with .devcontainer/.env (that file wins when found) ----
declare -A V=(
  [RUSTFMT_VERSION]=nightly-2026-11-06
  [CARGO_BINSTALL_VERSION]=1.26.2
  [CARGO_MAKE_VERSION]=0.37.24
  [CARGO_NEXTEST_VERSION]=0.9.148
  [CARGO_DENY_VERSION]=0.20.2
  [CARGO_MACHETE_VERSION]=0.9.2
  [CARGO_LLVM_COV_VERSION]=0.9.1
  [ZIZMOR_VERSION]=1.32.0
  [SCCACHE_VERSION]=0.19.0
  [DPRINT_VERSION]=0.64.3
  [LYCHEE_VERSION]=0.24.2
  [TYPOS_VERSION]=1.52.0
  [ZOLA_VERSION]=0.23.6
  [CARGO_CHEF_VERSION]=0.1.72
)
# Unpinned tools resolve to the latest release unless <TOOL>_VERSION is set:
#   UV_VERSION JUST_VERSION PREK_VERSION MDBOOK_VERSION TRUNK_VERSION ...
# Default Rust toolchain when the repo has no rust-toolchain.toml (RUST_VERSION or "stable").

ALL_GROUPS=(apt rust cargo fmt python just extras web docs gh)
DEFAULT_GROUPS=(rust cargo fmt python just)

dry_run=${SETUP_DRY_RUN:-0}
only=${SETUP_COMPONENTS:-}
with=${SETUP_WITH:-}
skip=${SETUP_SKIP:-}
failures=()

usage() { sed -n '2,/^set -euo/{/^set -euo/!s/^# \{0,1\}//p}' "${BASH_SOURCE[0]}"; }

while [[ $# -gt 0 ]]; do
  case "$1" in
    --only) only=${2:?--only needs a value}; shift 2 ;;
    --with) with=${2:?--with needs a value}; shift 2 ;;
    --skip) skip=${2:?--skip needs a value}; shift 2 ;;
    --dry-run) dry_run=1; shift ;;
    --list) printf '%s\n' "${ALL_GROUPS[@]}"; exit 0 ;;
    -h | --help) usage; exit 0 ;;
    *) printf 'Unknown option: %s\n' "$1" >&2; exit 2 ;;
  esac
done

if [[ $(uname -s) != Linux ]]; then
  printf 'This script supports Linux only (use scripts/install-tools.ps1 on Windows).\n' >&2
  exit 1
fi

# ---- Selection ----
# Resolve requested groups first, so --only, --with, and --skip compose predictably.
# Selection precedence is defaults, then additions, then removals.
declare -A selected=()
split() { local IFS=,; read -r -a _out <<<"$1"; printf '%s\n' "${_out[@]}"; }
add_group() {
  case "$1" in
    all) for g in "${ALL_GROUPS[@]}"; do selected[$g]=1; done ;;
    default) for g in "${DEFAULT_GROUPS[@]}"; do selected[$g]=1; done ;;
    apt | rust | cargo | fmt | python | just | extras | web | docs | gh) selected[$1]=1 ;;
    '') ;;
    *) printf 'Unknown group: %s (see --list)\n' "$1" >&2; exit 2 ;;
  esac
}
if [[ -z $only ]]; then only=default; fi
while IFS= read -r g; do add_group "$g"; done < <(split "$only")
while IFS= read -r g; do add_group "$g"; done < <(split "$with")
while IFS= read -r g; do [[ -n $g ]] && unset "selected[$g]"; done < <(split "$skip")
want() { [[ -n ${selected[$1]:-} ]]; }

# ---- Version overrides: env file, then environment ----
# Explicit process variables take precedence over both the env file and the built-in pins.
env_file=${ENV_FILE:-}
if [[ -z $env_file && -f ./.devcontainer/.env ]]; then env_file=./.devcontainer/.env; fi
if [[ -n $env_file && -f $env_file ]]; then
  while IFS= read -r line || [[ -n $line ]]; do
    line=${line%$'\r'}
    if [[ $line =~ ^([A-Z][A-Z0-9_]*_VERSION)=([A-Za-z0-9._-]+)$ ]]; then V[${BASH_REMATCH[1]}]=${BASH_REMATCH[2]}; fi
  done <"$env_file"
fi
for k in "${!V[@]}"; do if [[ -n ${!k:-} ]]; then V[$k]=${!k}; fi; done
ver() { printf '%s' "${V[$1]:-${!1:-}}"; }

# ---- Helpers ----
# Keep each external command visible so --dry-run output can be copied and reviewed.
log() { printf '\n==> %s\n' "$*"; }
run() {
  printf '> '; printf '%q ' "$@"; printf '\n'
  if [[ $dry_run == 1 ]]; then return 0; fi
  "$@"
}
# step <label> <cmd...>: run, record failure, keep going.
# Continue through independent groups, then report all failed steps in the final summary.
step() {
  local label=$1; shift
  if ! run "$@"; then failures+=("$label"); printf '!! failed: %s\n' "$label" >&2; fi
}
have() { command -v "$1" >/dev/null 2>&1; }
# Support both root containers and regular Linux accounts that have sudo.
sudo_cmd() {
  if [[ $(id -u) -eq 0 ]]; then "$@"
  elif have sudo; then sudo "$@"
  else printf 'need root for: %s\n' "$*" >&2; return 1; fi
}

export CARGO_HOME=${CARGO_HOME:-$HOME/.cargo}
export RUSTUP_HOME=${RUSTUP_HOME:-$HOME/.rustup}
export PATH="$CARGO_HOME/bin:$HOME/.local/bin:$PATH"
export RUSTC_WRAPPER=   # sccache may not exist yet; ignore any inherited wrapper while installing
bindir=$CARGO_HOME/bin

# Install cargo tools via binstall (prebuilt), falling back to compiling.
# Args: crate[@version] ... ; idempotent (binstall skips up-to-date installs).
binstall() {
  if [[ $dry_run != 1 ]] && ! have cargo-binstall; then
    printf 'cargo-binstall missing; enable the "cargo" group.\n' >&2
    return 1
  fi
  local pkg
  for pkg in "$@"; do
    step "$pkg" cargo binstall --no-confirm --locked "$pkg"
  done
}
pin() { printf '%s@%s' "$1" "$(ver "$2")"; }

# ---- apt ----
if want apt || want fmt || want web; then
  pkgs=()
  want apt && pkgs+=(build-essential clang lld mold cmake pkg-config libssl-dev libsqlite3-dev protobuf-compiler git curl ca-certificates xz-utils unzip)
  want fmt && pkgs+=(shellcheck)
  want web && pkgs+=(binaryen)
  if ((${#pkgs[@]})); then
    log "System packages: ${pkgs[*]}"
    if have apt-get; then
      step apt-get-update sudo_cmd env DEBIAN_FRONTEND=noninteractive apt-get update -qq
      step apt-install sudo_cmd env DEBIAN_FRONTEND=noninteractive apt-get install -y -qq --no-install-recommends "${pkgs[@]}"
    else
      printf 'apt-get not found; install manually: %s\n' "${pkgs[*]}" >&2
    fi
  fi
fi

# ---- rust ----
if want rust; then
  log "Rust toolchain"
  if ! have rustup; then
    if [[ $dry_run == 1 ]]; then
      echo '> curl https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none'
    else
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none
    fi
  fi
  # A repo rust-toolchain.toml (the cwd's) wins; otherwise RUST_VERSION or stable.
  default_toolchain=${RUST_VERSION:-stable}
  step rust-stable rustup toolchain install "$default_toolchain" --profile minimal --component clippy --component rustfmt
  step rust-default rustup default "$default_toolchain"
  step llvm-tools rustup component add llvm-tools-preview
  step nightly-rustfmt rustup toolchain install "$(ver RUSTFMT_VERSION)" --profile minimal --component rustfmt
fi

# ---- cargo ----
if want cargo; then
  log "Cargo tools"
  if ! have cargo-binstall || [[ $(cargo-binstall -V 2>/dev/null || true) != "$(ver CARGO_BINSTALL_VERSION)" ]]; then
    # Prebuilt binstall from its release; fall back to compiling.
    arch=$(uname -m)
    if [[ $dry_run == 1 ]]; then
      echo "> install cargo-binstall $(ver CARGO_BINSTALL_VERSION) (prebuilt, fallback: cargo install)"
    elif ! (tmp=$(mktemp -d) && curl -fsSL "https://github.com/cargo-bins/cargo-binstall/releases/download/v$(ver CARGO_BINSTALL_VERSION)/cargo-binstall-${arch}-unknown-linux-musl.tgz" | tar -xz -C "$tmp" \
        && mkdir -p "$bindir" && install -m 0755 "$tmp/cargo-binstall" "$bindir/cargo-binstall"); then
      step cargo-binstall cargo install --locked cargo-binstall --version "$(ver CARGO_BINSTALL_VERSION)"
    fi
  fi
  binstall "$(pin cargo-make CARGO_MAKE_VERSION)" "$(pin cargo-nextest CARGO_NEXTEST_VERSION)" \
    "$(pin cargo-deny CARGO_DENY_VERSION)" "$(pin cargo-machete CARGO_MACHETE_VERSION)" \
    "$(pin cargo-llvm-cov CARGO_LLVM_COV_VERSION)" "$(pin sccache SCCACHE_VERSION)"
fi

# ---- fmt / lint ----
if want fmt; then
  log "Format, lint and hook tools"
  binstall "$(pin dprint DPRINT_VERSION)" "$(pin lychee LYCHEE_VERSION)" "$(pin typos-cli TYPOS_VERSION)" \
    "$(pin zizmor ZIZMOR_VERSION)" "prek${PREK_VERSION:+@$PREK_VERSION}"
fi

# ---- python ----
if want python; then
  log "uv (latest)"
  if [[ -n ${UV_VERSION:-} ]]; then uv_url="https://astral.sh/uv/${UV_VERSION}/install.sh"; else uv_url="https://astral.sh/uv/install.sh"; fi
  if have uv && [[ -z ${UV_VERSION:-} ]]; then
    step uv-self-update uv self update
  elif [[ $dry_run == 1 ]]; then
    echo "> curl -LsSf $uv_url | sh"
  else
    step uv bash -c "curl -LsSf '$uv_url' | env UV_NO_MODIFY_PATH=1 UV_INSTALL_DIR='$HOME/.local/bin' sh"
  fi
fi

# ---- just ----
if want just; then
  log "just"
  binstall "just${JUST_VERSION:+@$JUST_VERSION}"
fi

# ---- extras ----
if want extras; then
  log "Extra cargo tools"
  binstall cargo-audit cargo-sort cargo-outdated cargo-udeps cargo-hack cargo-insta cargo-fuzz \
    "$(pin cargo-chef CARGO_CHEF_VERSION)" cargo-generate cargo-watch
fi

# ---- web ----
if want web; then
  log "Web / wasm tools"
  step wasm-target rustup target add wasm32-unknown-unknown
  binstall trunk wasm-bindgen-cli dioxus-cli
fi

# ---- docs ----
if want docs; then
  log "Docs tools"
  zola_version=$(ver ZOLA_VERSION)
  if have zola && [[ $(zola --version 2>/dev/null) == "zola $zola_version" ]]; then
    echo "zola $zola_version already installed"
  else
    case "$(uname -m)" in
      x86_64) platform=x86_64-unknown-linux-gnu ;;
      aarch64) platform=aarch64-unknown-linux-gnu ;;
      *) platform= ;;
    esac
    case "$zola_version/$platform" in
      0.23.6/x86_64-unknown-linux-gnu) sum=8f5132b3522412d04e395e0b25f6d68613ad272a873e54a2b3ebf664873024a4 ;;
      0.23.6/aarch64-unknown-linux-gnu) sum=266448fffbf7c7004ca399d0e76dd699541771096d8a42aede98cebe2a029d02 ;;
      *) sum= ;;
    esac
    if [[ -z $sum ]]; then
      printf 'No pinned SHA-256 for zola %s on %s; add one to this script.\n' "$zola_version" "${platform:-?}" >&2
      failures+=(zola)
    elif [[ $dry_run == 1 ]]; then
      echo "> download zola $zola_version ($platform), verify SHA-256, install to $bindir"
    else
      tmp=$(mktemp -d)
      asset=zola-v${zola_version}-${platform}.tar.gz
      if curl -fsSL "https://github.com/getzola/zola/releases/download/v${zola_version}/${asset}" -o "$tmp/$asset" \
        && printf '%s  %s\n' "$sum" "$tmp/$asset" | sha256sum --check --status \
        && tar -xzf "$tmp/$asset" -C "$tmp" && mkdir -p "$bindir" && install -m 0755 "$tmp/zola" "$bindir/zola"; then
        echo "installed zola $zola_version"
      else
        failures+=(zola)
        echo '!! failed: zola (download or checksum)' >&2
      fi
      rm -rf "$tmp"
    fi
  fi
  binstall "mdbook${MDBOOK_VERSION:+@$MDBOOK_VERSION}" mdbook-admonish mdbook-linkcheck
fi

# ---- gh / actionlint ----
if want gh; then
  log "GitHub CLI and actionlint"
  if have gh; then
    echo "gh already installed"
  elif have apt-get; then
    step gh sudo_cmd env DEBIAN_FRONTEND=noninteractive apt-get install -y -qq gh
  elif have dnf; then
    step gh sudo_cmd dnf install -y gh
  else
    echo 'Install gh manually (no supported package manager).' >&2
  fi
  if have actionlint; then
    echo "actionlint already installed"
  elif [[ $dry_run == 1 ]]; then
    echo "> install actionlint (latest release) to $bindir"
  else
    step actionlint bash -c "cd \"\$(mktemp -d)\" && bash <(curl -fsSL https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash) latest && install -m 0755 actionlint '$bindir/actionlint'"
  fi
fi

# ---- PATH for later shells ----
if [[ $dry_run != 1 ]]; then
  line='export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"'
  for rc in "$HOME/.bashrc" "$HOME/.profile"; do
    if [[ -f $rc ]] && ! grep -qF "$line" "$rc"; then printf '%s\n' "$line" >>"$rc"; fi
  done
fi

log "Summary"
for t in rustc cargo cargo-make cargo-nextest cargo-deny dprint lychee typos prek uv just zola mdbook; do
  if have "$t"; then printf '%-16s %s\n' "$t" "$("$t" --version 2>/dev/null | head -n1)"; fi
done
if ((${#failures[@]})); then
  printf '\nFAILED: %s\n' "${failures[*]}" >&2
  exit 1
fi
printf '\nDone. Open a new shell or: export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"\n'

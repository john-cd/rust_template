#!/usr/bin/env bash

# Download a supported Zola release, verify its pinned checksum, and install it to the requested directory.
# Arguments are the exact Zola version and destination bin directory; unsupported targets fail before download.
set -euo pipefail

version=${1:?Usage: install-zola.sh <version> <destination>}
destination=${2:?Usage: install-zola.sh <version> <destination>}

system=$(uname -s)
architecture=$(uname -m)
# Only map platforms with release artifacts supported by this installer.
case "$system/$architecture" in
  Linux/x86_64) platform=x86_64-unknown-linux-gnu ;;
  Linux/aarch64) platform=aarch64-unknown-linux-gnu ;;
  *) printf 'Unsupported Zola platform: %s/%s\n' "$system" "$architecture" >&2; exit 1 ;;
esac

asset="zola-v${version}-${platform}.tar.gz"
# Require a checked-in checksum for the exact version and platform before installing.
case "$version/$platform" in
  0.23.6/x86_64-unknown-linux-gnu) checksum=8f5132b3522412d04e395e0b25f6d68613ad272a873e54a2b3ebf664873024a4 ;;
  0.23.6/aarch64-unknown-linux-gnu) checksum=266448fffbf7c7004ca399d0e76dd699541771096d8a42aede98cebe2a029d02 ;;
  *) printf 'No pinned Zola release checksum for %s/%s.\n' "$version" "$platform" >&2; exit 1 ;;
esac

archive_url="https://github.com/getzola/zola/releases/download/v${version}/${asset}"
temp_dir=$(mktemp -d)
# Remove the archive and extracted files after success, failure, or interruption.
trap 'rm -rf "$temp_dir"' EXIT
archive="$temp_dir/$asset"

curl --fail --location --silent --show-error "$archive_url" --output "$archive"
# Verify the release archive before extracting or installing its executable.
printf '%s  %s\n' "$checksum" "$archive" | sha256sum --check --status

mkdir -p "$destination"
tar --extract --gzip --file "$archive" --directory "$temp_dir"
install --mode=0755 "$temp_dir/zola" "$destination/zola"

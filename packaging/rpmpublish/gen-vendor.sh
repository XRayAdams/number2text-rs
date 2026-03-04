#!/bin/bash
# Generates the offline Cargo vendor tarball required as Source1 in the spec.
# Run this script from any directory; output is placed next to the script.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPEC="$SCRIPT_DIR/app.rayadams.number2text.spec"

# Read upstreamver from the spec file
UPSTREAMVER=$(grep -Po '(?<=^%global upstreamver ).*' "$SPEC" | tr -d '[:space:]')
NAME="number2text"
# GitHub replaces '+' with '-' inside the extracted tarball directory name
SRC_DIR_IN_TAR="$NAME-rs-${UPSTREAMVER//+/-}"
SRC_TARBALL="$NAME-rs-$UPSTREAMVER.tar.gz"
VENDOR_TARBALL="$SCRIPT_DIR/$NAME-$UPSTREAMVER-vendor.tar.xz"
GITHUB_URL="https://github.com/XRayAdams/number2text-rs/archive/refs/tags/v$UPSTREAMVER/$SRC_TARBALL"

echo "==> upstreamver: $UPSTREAMVER"
echo "==> Output:      $VENDOR_TARBALL"

if [[ -f "$VENDOR_TARBALL" ]]; then
    echo "==> Vendor tarball already exists, skipping."
    exit 0
fi

WORKDIR=$(mktemp -d)
trap 'rm -rf "$WORKDIR"' EXIT

echo "==> Downloading source tarball..."
curl -fSL "$GITHUB_URL" -o "$WORKDIR/$SRC_TARBALL"

echo "==> Extracting..."
tar xf "$WORKDIR/$SRC_TARBALL" -C "$WORKDIR"

echo "==> Running cargo vendor..."
cd "$WORKDIR/$SRC_DIR_IN_TAR"
cargo vendor vendor

echo "==> Creating vendor tarball..."
tar cJf "$VENDOR_TARBALL" vendor/

echo "==> Done: $VENDOR_TARBALL"

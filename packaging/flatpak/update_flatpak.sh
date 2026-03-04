#!/bin/bash
# Run this AFTER the GitHub release tag has been pushed.

set -e

echo "___________________________________________________________"
echo "Updating flatpak manifest and metainfo for new release..."

# --- Configuration ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
CARGO_FILE="$PROJECT_ROOT/Cargo.toml"
FLATPAK_YML_FILE="$SCRIPT_DIR/app.rayadams.number2text.yml"
METAINFO_FILE="$PROJECT_ROOT/packaging/app.rayadams.number2text.metainfo.xml"
# ---------------------

# Check files exist
for f in "$CARGO_FILE" "$FLATPAK_YML_FILE" "$METAINFO_FILE"; do
    if [ ! -f "$f" ]; then
        echo "Error: File not found: $f"
        exit 1
    fi
done

# Read version from Cargo.toml
APP_VERSION=$(grep -E '^\s*version = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')
if [ -z "$APP_VERSION" ]; then
    echo "Error: Could not read version from $CARGO_FILE."
    exit 1
fi
echo "Version: $APP_VERSION"

TARBALL_URL="https://github.com/XRayAdams/number2text-rs/archive/refs/tags/v${APP_VERSION}/number2text-rs-${APP_VERSION}.tar.gz"

# Download tarball and compute sha256
echo "Downloading tarball: $TARBALL_URL"
NEW_SHA256=$(curl -fSL "$TARBALL_URL" | sha256sum | cut -d ' ' -f 1)
if [ -z "$NEW_SHA256" ]; then
    echo "Error: Could not compute sha256. Is the tag pushed to GitHub?"
    exit 1
fi
echo "sha256: $NEW_SHA256"

# Update flatpak manifest
sed -i "s|url: https://github.com/XRayAdams/number2text-rs/archive/refs/tags/v[^/]*/number2text-rs-[^.]*\.tar\.gz|url: $TARBALL_URL|" "$FLATPAK_YML_FILE"
sed -i "s|dest-filename: number2text-rs-.*\.tar\.gz|dest-filename: number2text-rs-${APP_VERSION}.tar.gz|" "$FLATPAK_YML_FILE"
sed -i "s|^\(\s*sha256: \).*|\1${NEW_SHA256}|" "$FLATPAK_YML_FILE"
echo "Updated: $FLATPAK_YML_FILE"

# Update metainfo <release> and screenshot URLs
RELEASE_DATE=$(date +"%Y-%m-%d")
sed -i "s|<release version=\".*\" date=\".*\"/>|<release version=\"${APP_VERSION}\" date=\"${RELEASE_DATE}\"/>|" "$METAINFO_FILE"
sed -i "s|/refs/tags/v[^/]*/screenshots/|/refs/tags/v${APP_VERSION}/screenshots/|g" "$METAINFO_FILE"
echo "Updated: $METAINFO_FILE"
echo "Done."

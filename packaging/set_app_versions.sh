#!/bin/bash

echo "___________________________________________________________"
echo "Setting app version in all relevant files..."


# --- Configuration ---
CARGO_FILE="Cargo.toml"
DEBIAN_CONTROL_FILE="packaging/control"
SNAP_YAML_FILE="snap/snapcraft.yaml"
RPM_FILE="packaging/number2text.spec"
FEDORA_SPEC_FILE="packaging/rpmpublish/app.rayadams.number2text.spec"
MACHINE_ARCH=$(uname -m)
DEBIAN_CONTROL_FILE_ARCH="amd64"
AUR_CONTROL_FILE="packaging/PKGBUILD"

if [ "$MACHINE_ARCH" == "aarch64" ]; then
    MACHINE_ARCH="arm64"
    DEBIAN_CONTROL_FILE_ARCH="arm64"
    echo "Architecture was aarch64, updated to: $MACHINE_ARCH"
elif [ "$MACHINE_ARCH" == "x86_64" ]; then
    MACHINE_ARCH="x64"
    DEBIAN_CONTROL_FILE_ARCH="amd64"
fi
# ---------------------


# Check if files exist
if [ ! -f "$CARGO_FILE" ]; then
    echo "Error: File not found: $CARGO_FILE"
    exit 1
fi
if [ ! -f "$DEBIAN_CONTROL_FILE" ]; then
    echo "Error: File not found: $DEBIAN_CONTROL_FILE"
    exit 1
fi
if [ ! -f "$SNAP_YAML_FILE" ]; then
    echo "Error: File not found: $SNAP_YAML_FILE"
    exit 1
fi
if [ ! -f "$RPM_FILE" ]; then
    echo "Error: File not found: $RPM_FILE"
    exit 1
fi
if [ ! -f "$FEDORA_SPEC_FILE" ]; then
    echo "Error: File not found: $FEDORA_SPEC_FILE"
    exit 1
fi

# Read version from Cargo.toml (extracts the line with 'version =' and gets the value after the space)
APP_VERSION=$(grep -E '^\s*version = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')

if [ -z "$APP_VERSION" ]; then
    echo "Error: Could not read version from $CARGO_FILE."
    exit 1
fi

echo "Version '$APP_VERSION' found in $CARGO_FILE"

# Parse version into APP_VERSION_SHORT and APP_BUILD
APP_VERSION_SHORT=$(echo "$APP_VERSION" | cut -d'+' -f1)
APP_BUILD=$(echo "$APP_VERSION" | cut -d'+' -f2)

# Use sed to find and replace the Version line in debian.yaml and snap
# This command looks for the line starting with '  Version:' and replaces the entire line.
sed -i "s/^\(\s*Version:\s*\).*\$/\1$APP_VERSION/" "$DEBIAN_CONTROL_FILE"
sed -i "s/^\(\s*Architecture:\s*\).*\$/\1$DEBIAN_CONTROL_FILE_ARCH/" "$DEBIAN_CONTROL_FILE"
sed -i "s/^\(\s*version:\s*\).*\$/\1$APP_VERSION/" "$SNAP_YAML_FILE"

# Update version in RPM spec file
sed -i "s/^\(\s*%define _version \s*\).*\$/\1$APP_VERSION_SHORT/" "$RPM_FILE"
sed -i "s/^\(\s*%define _release \s*\).*\$/\1$APP_BUILD/" "$RPM_FILE"

# Update version in Fedora spec file
CHANGELOG_DATE=$(date +"%a %b %d %Y")
CHANGELOG_VER="$APP_VERSION_SHORT-$APP_BUILD"
sed -i "s/^\(%global upstreamver \).*\$/\1$APP_VERSION/" "$FEDORA_SPEC_FILE"
sed -i "s/^\(Version:\s*\).*\$/\1$APP_VERSION_SHORT/" "$FEDORA_SPEC_FILE"
sed -i "s/^\(Release:\s*\).*\$/\1${APP_BUILD}%{?dist}/" "$FEDORA_SPEC_FILE"
sed -i "s/^\(\* \).*\(<xrayadamo@gmail\.com>\).*\$/\1$CHANGELOG_DATE Konstantin Adamov \2 - $CHANGELOG_VER/" "$FEDORA_SPEC_FILE"

# Update version in AUR PKGBUILD
sed -i "s/^\(\s*pkgver=\).*\$/\1$APP_VERSION_SHORT/" "$AUR_CONTROL_FILE"
sed -i "s/^\(\s*pkgrel=\).*\$/\1$APP_BUILD/" "$AUR_CONTROL_FILE"

echo "Successfully updated version to $APP_VERSION in all relevant files."

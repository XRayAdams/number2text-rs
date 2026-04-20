#!/bin/bash
set -e

# --- Configuration ---
MACHINE_ARCH=$(uname -m)
APPIMAGE_ARCH=$MACHINE_ARCH
APP_ID="app.rayadams.number2text"
CARGO_FILE="Cargo.toml"

if [ "$MACHINE_ARCH" == "aarch64" ]; then
    MACHINE_ARCH="arm64"
fi


# Clean and build app in release mode
cargo clean
cargo build --release
rm -rf dist

APP_NAME=$(grep -E '^\s*name = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')
APP_VERSION_LONG=$(grep -E '^\s*version = ' "$CARGO_FILE" | head -n1 | cut -d ' ' -f 3 | tr -d '"')
APP_VERSION=$(echo "$APP_VERSION_LONG" | cut -d'+' -f1)
APP_BUILD=$(echo "$APP_VERSION_LONG" | cut -d'+' -f2)

strip target/release/"$APP_NAME"

# Set app versions to all files for packaging
packaging/set_app_versions.sh
mkdir -p dist

cargo deb --output dist

echo "DEB package created in dist/"
echo "___________________________________________________________"
# Package RPM
echo "Preparing RPM package"

# Create RPM build directories
RPM_BUILD_ROOT="$(pwd)/rpmbuild"
rm -rf "$RPM_BUILD_ROOT"

mkdir -p "$RPM_BUILD_ROOT/BUILD"
mkdir -p "$RPM_BUILD_ROOT/RPMS"
mkdir -p "$RPM_BUILD_ROOT/SOURCES"
mkdir -p "$RPM_BUILD_ROOT/SPECS"
mkdir -p "$RPM_BUILD_ROOT/SRPMS"

CHANGE_DATE=$(date +"%a %b %d %Y")
CHANGE_DATE="$CHANGE_DATE Konstantin Adamov (xrayadamo@gmail.com) - $APP_VERSION-$APP_BUILD"
sed "s/^*loghere$/* $CHANGE_DATE/" "packaging/$APP_NAME.spec" > "$RPM_BUILD_ROOT/SPECS/$APP_NAME.spec"

# Copy desktop and icon files, replacing Exec and TryExec with app name , by default it has full path for debian package
sed -e "s/Icon=$APP_ID/Icon=$APP_NAME/" -e "s/^\(Exec\|TryExec\)=.*$/\1=$APP_NAME/" "packaging/gui/$APP_ID.desktop"  > "$RPM_BUILD_ROOT/SOURCES/$APP_ID.desktop"
cp packaging/gui/"$APP_ID".png "$RPM_BUILD_ROOT/SOURCES/"
cp packaging/"$APP_ID".metainfo.xml "$RPM_BUILD_ROOT/SOURCES/"
cp assets/number2text.1.gz "$RPM_BUILD_ROOT/SOURCES/"

# Package the application files into a tarball
pushd target || exit
tar -czvf "$RPM_BUILD_ROOT/SOURCES/$APP_NAME-$APP_VERSION.tar.gz" "release/$APP_NAME" 
popd || exit

# Build the RPM
rpmbuild -bb \
    --define "_topdir $RPM_BUILD_ROOT" \
    --define "_name $APP_NAME" \
    --define "_version $APP_VERSION" \
    --define "_release $APP_BUILD" \
    "$RPM_BUILD_ROOT/SPECS/$APP_NAME.spec"

# Move the RPM to the dist directory
find "$RPM_BUILD_ROOT/RPMS" -name "*.rpm" -exec mv {} dist/ \;

# Clean up
rm -rf "$RPM_BUILD_ROOT"
echo "RPM package created in dist/"
echo "___________________________________________________________"

#Packaging AUR
cp packaging/PKGBUILD .
PACKAGER="Konstantin Adamov <xrayadamo@gmail.com>" PKGEXT='.pkg.tar.zst' COMPRESSZST=(zstd -c -T0 --auto-threads=logical -) env makepkg --nodeps -f
mv *.pkg.tar.zst dist/
rm -rf pkg/
rm PKGBUILD

echo "AUR package created in dist/"
echo "___________________________________________________________"

# Package TAR
echo "Preparing TAR archive"

ARCHIVE_NAME="${APP_NAME}-${APP_VERSION}+${APP_BUILD}-${MACHINE_ARCH}.tar.gz"
FULL_ARCHIVE_PATH="dist/${ARCHIVE_NAME}"
SOURCE_DIR="target/release"

tar -czvf "$FULL_ARCHIVE_PATH" -C "$SOURCE_DIR" "$APP_NAME" -C "$(pwd)/assets" "number2text.1.gz" > /dev/null
echo "TAR archive created in dist/"
echo "___________________________________________________________"

echo "Creating AppImage"
APP_IMAGE_FOLDER="AppImage"
LINUXDEPLOY_TOOL="linuxdeploy-x86_64.AppImage"
APPIMAGE_TOOL="appimagetool-x86_64.AppImage"
APPIMAGE_OUTPUT="${APP_NAME}-${APP_VERSION}+${APP_BUILD}-${MACHINE_ARCH}.AppImage"

rm -rf "$APP_IMAGE_FOLDER"

mkdir -p "$APP_IMAGE_FOLDER/usr/share/applications"
mkdir -p "$APP_IMAGE_FOLDER/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$APP_IMAGE_FOLDER/usr/share/metainfo"

cp "packaging/gui/${APP_ID}.desktop"    "$APP_IMAGE_FOLDER/usr/share/applications/"
cp "packaging/gui/${APP_ID}.png"        "$APP_IMAGE_FOLDER/usr/share/icons/hicolor/512x512/apps/"
cp "packaging/${APP_ID}.metainfo.xml"   "$APP_IMAGE_FOLDER/usr/share/metainfo/$APP_ID.appdata.xml"
cp "packaging/gui/${APP_ID}.desktop"    "$APP_IMAGE_FOLDER/"
cp "packaging/gui/${APP_ID}.png"        "$APP_IMAGE_FOLDER/"

ARCH="$MACHINE_ARCH" NO_STRIP=1 \
"$LINUXDEPLOY_TOOL" \
    --appdir "$APP_IMAGE_FOLDER" \
    --executable "target/release/$APP_NAME" \
    --plugin gtk

# linuxdeploy's bundled patchelf is too old for modern RELR relocations and corrupts libs.
# Replace every patched lib with the original system copy.
for LIB in "$APP_IMAGE_FOLDER"/usr/lib/*.so*; do
    [ -f "$LIB" ] || continue
    LIBNAME=$(basename "$LIB")
    SYSPATH=$(ldconfig -p | grep -m1 "[[:space:]]${LIBNAME}[[:space:]]" | awk -F'=> ' '{print $2}')
    if [ -n "$SYSPATH" ] && [ -f "$SYSPATH" ]; then
        cp "$SYSPATH" "$LIB"
    fi
done

# Add LD_LIBRARY_PATH to AppRun so bundled (now unpatched) libs are found
sed -i "s|^exec \"\$this_dir\"/AppRun.wrapped|export LD_LIBRARY_PATH=\"\$this_dir/usr/lib:\$LD_LIBRARY_PATH\"\nexec \"\$this_dir\"/AppRun.wrapped|" "$APP_IMAGE_FOLDER/AppRun"

# Remove libraries that must come from the host system
LIBS_TO_REMOVE=(
    libgstreamer-1.0.so.0 libgstbase-1.0.so.0 libgstaudio-1.0.so.0
    libgstvideo-1.0.so.0 libgstpbutils-1.0.so.0 libgstgl-1.0.so.0
    libgsttag-1.0.so.0 libgstallocators-1.0.so.0 libgstplay-1.0.so.0
    libcups.so.2 libcurl.so.4
    libavahi-client.so.3 libavahi-common.so.3
    libgssapi_krb5.so.2 libkrb5.so.3 libk5crypto.so.3 libkrb5support.so.0 libkeyutils.so.1
    liblber.so.2 libldap.so.2 libsasl2.so.3 libssh.so.4
    libdav1d.so.7 libtinysparql-3.0.so.0 libappstream.so.5 libcolord.so.2
    libvulkan.so.1 libsystemd.so.0 libudev.so.1 libgudev-1.0.so.0
    libselinux.so.1 libcap.so.2 libseccomp.so.2 libdbus-1.so.3
    libcrypt.so.2 libffi.so.8
    libmount.so.1 libblkid.so.1 libpcre2-8.so.0
    libelf.so.1 libdw.so.1 libunwind.so.8
    liblzma.so.5 libbz2.so.1 libzstd.so.1
    libgnutls.so.30 libssl.so.3 libcrypto.so.3
    libp11-kit.so.0 libtasn1.so.6 libhogweed.so.6 libnettle.so.8
    libunistring.so.5 libidn2.so.0 libpsl.so.5 libnghttp2.so.14 libevent-2.1.so.7
    liborc-0.4.so.0 libstemmer.so.0 libfyaml.so.0 libxmlb.so.2
    libwebp.so.7 libsharpyuv.so.0 libwayland-egl.so.1
)
for LIB in "${LIBS_TO_REMOVE[@]}"; do
    rm -f "$APP_IMAGE_FOLDER/usr/lib/$LIB"
done

# Remove bundled GTK input method modules (use system ones at runtime)
rm -rf "$APP_IMAGE_FOLDER/usr/lib/gtk-4.0"

# Fix the gtk plugin hook: it forces GTK_THEME=Adwaita which breaks libadwaita styling,
# forces GDK_BACKEND=x11 which breaks Wayland, and sets GTK_PATH to the deleted gtk-4.0 dir.
HOOK="$APP_IMAGE_FOLDER/apprun-hooks/linuxdeploy-plugin-gtk.sh"
sed -i \
    '/^export GTK_THEME=/d' \
    "$HOOK"
sed -i \
    '/^export GDK_BACKEND=x11/d' \
    "$HOOK"
sed -i \
    '/^export GTK_PATH=/d' \
    "$HOOK"

ARCH="$MACHINE_ARCH" "$APPIMAGE_TOOL" "$APP_IMAGE_FOLDER" "dist/$APPIMAGE_OUTPUT" \
    --comp zstd --mksquashfs-opt -Xcompression-level --mksquashfs-opt 22

rm -rf "$APP_IMAGE_FOLDER"
echo "AppImage created: dist/$APPIMAGE_OUTPUT"

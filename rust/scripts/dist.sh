#!/bin/sh
# Builds the multi-platform release zip: target/dist/Tearout-Farm-<version>.zip
# Mac universal builds here; Linux (x86_64, arm64) and Windows build on gx10-3, which holds
# rustup, zig, cargo-zigbuild, cargo-xwin and the Ubuntu focal sysroots ~/x86root and ~/armroot.
set -e
cd "$(dirname "$0")/.."
H=${H:-gx10-3}
V=$(grep -m1 '^version' tearout-plug/Cargo.toml | cut -d'"' -f2)
nice cargo xtask bundle-universal tearout-plug --release >/dev/null
rsync -az --exclude target ./ $H:tf/rust/
rsync -az ../web/presets.json ../web/priors.json ../web/gunness.json $H:tf/web/
ssh $H 'set -e; . ~/.cargo/env; export PATH=~/opt/zig:$PATH CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_STRIP=symbols PKG_CONFIG_ALLOW_CROSS=1
cd ~/tf/rust
rm -f target/*/release/libtearout_plug.so target/*/release/tearout_plug.dll
lin() { R=$1; L=$2; T=$3; PKG_CONFIG_SYSROOT_DIR=$R PKG_CONFIG_LIBDIR=$R/usr/lib/$L/pkgconfig:$R/usr/share/pkgconfig RUSTFLAGS="-L $R/usr/lib/$L -L $R/lib/$L" \
  nice cargo zigbuild -q --release -j 8 -p tearout-plug --lib --no-default-features --target $T.2.31 2>/dev/null; }
lin ~/x86root x86_64-linux-gnu x86_64-unknown-linux-gnu
lin ~/armroot aarch64-linux-gnu aarch64-unknown-linux-gnu
RUSTFLAGS="-C target-feature=+crt-static" nice cargo xwin build -q --release -j 8 -p tearout-plug --lib --no-default-features --target x86_64-pc-windows-msvc 2>/dev/null'
O="target/dist/pkg/Tearout Farm"; B="$O/Tearout Farm.vst3"
rm -rf target/dist/pkg; mkdir -p "$O"
cp -R "target/bundled/Tearout Farm.vst3" "$B"; rm -rf "$B/Contents/_CodeSignature"
mkdir -p "$B/Contents/x86_64-win" "$B/Contents/x86_64-linux" "$B/Contents/aarch64-linux"
scp -q $H:tf/rust/target/x86_64-pc-windows-msvc/release/tearout_plug.dll "$B/Contents/x86_64-win/Tearout Farm.vst3"
scp -q $H:tf/rust/target/x86_64-unknown-linux-gnu/release/libtearout_plug.so "$B/Contents/x86_64-linux/Tearout Farm.so"
scp -q $H:tf/rust/target/aarch64-unknown-linux-gnu/release/libtearout_plug.so "$B/Contents/aarch64-linux/Tearout Farm.so"
cp scripts/linux/flake.nix scripts/linux/INSTALL.txt "$O/"
codesign --force --sign - "$B" && codesign --verify --strict "$B"
rm -f "target/dist/Tearout-Farm-$V.zip"
(cd target/dist/pkg && ditto -c -k --keepParent "Tearout Farm" "../Tearout-Farm-$V.zip")
echo "target/dist/Tearout-Farm-$V.zip"

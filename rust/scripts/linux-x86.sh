#!/bin/sh
# Cross-builds the x86_64 Linux bundles on gx10-3 and copies the tarball to target/dist.
# gx10-3 holds the toolchain (rustup, cargo-zigbuild, ~/opt/zig) and an Ubuntu focal sysroot in ~/x86root.
set -e
cd "$(dirname "$0")/.."
H=${H:-gx10-3}
rsync -az --exclude target ./ $H:tf/rust/
rsync -az scripts/linux/flake.nix $H:tf/
rsync -az ../web/presets.json ../web/priors.json ../web/gunness.json $H:tf/web/
ssh $H '. ~/.cargo/env; export PATH=~/opt/zig:$PATH; R=~/x86root
export PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_SYSROOT_DIR=$R PKG_CONFIG_LIBDIR=$R/usr/lib/x86_64-linux-gnu/pkgconfig:$R/usr/share/pkgconfig
export RUSTFLAGS="-L $R/usr/lib/x86_64-linux-gnu -L $R/lib/x86_64-linux-gnu" CARGO_PROFILE_RELEASE_DEBUG=0 CARGO_PROFILE_RELEASE_STRIP=symbols
cd ~/tf/rust && nice cargo zigbuild -q --release -j 8 -p tearout-plug --lib --no-default-features --target x86_64-unknown-linux-gnu.2.31
S=target/x86_64-unknown-linux-gnu/release/libtearout_plug.so; O="$HOME/tf/out/Tearout Farm"; rm -rf ~/tf/out
mkdir -p "$O/Tearout Farm.vst3/Contents/x86_64-linux"; cp $S "$O/Tearout Farm.clap"; cp $S "$O/Tearout Farm.vst3/Contents/x86_64-linux/Tearout Farm.so"; cp ~/tf/flake.nix "$O/"
cd ~/tf/out && tar czf tearout-farm-linux-x86_64.tar.gz "Tearout Farm"'
mkdir -p target/dist && scp -q $H:tf/out/tearout-farm-linux-x86_64.tar.gz target/dist/
echo target/dist/tearout-farm-linux-x86_64.tar.gz

#!/bin/bash
ARCH="aarch64"
TARGET="$ARCH-unknown-linux-gnu"
TARGET_DIR="./target/$TARGET/release"

echo Build for $ARCH
cargo build --release --target $TARGET && \

\cp "$TARGET_DIR/ya-runtime-vm-$ARCH-host"  ../yagna-binaries/plugins/ya-runtime-vm-aarch64-host/ya-runtime-vm-$ARCH-host
\cp "$TARGET_DIR/libya_runtime_vm_${ARCH}_host.rlib" ../yagna-binaries/
\cp "$TARGET_DIR/gvmkit" ../yagna-binaries/


#!/bin/bash


ARCH="aarch64"
TARGET="$ARCH-unknown-linux-gnu"

TARGET_DIR="./target/$TARGET/release"

echo -e "# build binary"
#PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true \
cargo build --release --target $TARGET && \

echo -e $(readelf --arch-specific "$TARGET_DIR/ya-runtime-vm-$ARCH-host")

\cp "$TARGET_DIR/ya-runtime-vm-$ARCH-host"  ../yagna-binaries/plugins/ya-runtime-vm-aarch64-host/ya-runtime-vm-$ARCH-host
\cp "$TARGET_DIR/libya_runtime_vm_${ARCH}_host.rlib" ../yagna-binaries/
\cp "$TARGET_DIR/gvmkit" ../yagna-binaries/


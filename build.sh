#!/bin/bash


#ARCH="aarch64"
#TARGET="$ARCH-unknown-linux-gnu"

ARCH="armv7"
TARGET="$ARCH-unknown-linux-gnueabihf"

TARGET_DIR="./target/$TARGET/release/"

echo -e "# build binary"
PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true \
cargo build --release --target $TARGET && \

echo -e $(readelf --arch-specific "$TARGET_DIR/ya-runtime-vm-aarch64-host") && \

\cp "$TARGET_DIR/ya-runtime-vm-aarch64-host"  ../yagna-binaries/$ARCH/ya-runtime-vm-$ARCH-host
\cp "$TARGET_DIR/libya_runtime_vm_aarch64_host.rlib" ../yagna-binaries/$ARCH/
\cp "$TARGET_DIR/gvmkit" ../yagna-binaries/$ARCH/


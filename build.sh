#!/bin/bash


TARGET="aarch64-unknown-linux-gnu"

PROJECT_NAME="ya-runtime-vm-aarch64-host"
TARGETFILE="./target/$TARGET/release/$PROJECT_NAME"

echo -e "# build binary"
#PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true \
cargo build --release --target $TARGET && \

echo -e $(readelf --arch-specific $TARGETFILE)

mv $TARGETFILE  ../yagna-binaries/aarch64

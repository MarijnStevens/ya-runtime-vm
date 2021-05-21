#!/bin/bash

ARCH=$1

if [ "$ARCH" = "" ] ; then
	echo requires architecture.
	exit 1
fi

export CROSS="--cross-prefix=aarch64-linux-gnu-"

#--disable-avx2 \
#--disable-avx512f \
#--disable-slirp \
#--disable-containers --disable-gtk \
#--disable-capstone \
#--disable-replication \
#--disable-parallels \
#--disable-sheepdog \
#--disable-vvfat \
#--disable-qed \
#--disable-vdi \
#--disable-qcow1 \
#--disable-dmg \
#--disable-cloop \
#--disable-bochs \
#--disable-bzip2 \
#--disable-guest-agent \
#--disable-numa \
#--disable-vnc \
#--disable-live-block-migration \

mkdir qemu-build && \
  cd qemu-build && \
  ../qemu-6.0.0/configure $CROSS \
  --static \
  --target-list=$ARCH-softmmu \
  --audio-drv-list="" \
  --enable-virtfs \
  --enable-vhost-vsock \
   --without-default-devices || exit 1

echo CONFIG_PARALLEL=y >> config-host.mak && \
echo CONFIG_VIRTIO_SERIAL=y >> config-host.mak && \
echo CONFIG_VIRTIO_PCI=y >> config-host.mak && \
echo CONFIG_VIRTIO_RNG=y >> config-host.mak && \
echo CONFIG_VIRTIO_MMIO=y >> config-host.mak && \
echo CONFIG_VIRTIO_SCSI=y >> config-host.mak && \
echo CONFIG_VIRTIO_BLK=y >> config-host.mak && \
echo CONFIG_VIRTIO_9P=y >> config-host.mak && \
echo CONFIG_FSDEV_9P=y >> config-host.mak

export CFLAGS="{CFLAGS} -Os -flto"
export LIBS="${LIBS} -flto -lblkid -luuid -lpixman-1 -lutil"

make -j$(nproc)
cp qemu-system-$ARCH ../vmrt-$ARCH

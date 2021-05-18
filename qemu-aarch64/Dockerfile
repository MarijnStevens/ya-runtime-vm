FROM debian:bullseye-slim

# Add the foreign architecture we want and install dependencies
RUN dpkg --add-architecture arm64
RUN apt update && \
    DEBIAN_FRONTEND=noninteractive \
        apt install -y --no-install-recommends \
        build-essential ca-certificates git make \
        python3 wget pkg-config  	libglib2.0-0 libglib2.0-dev libpixman-1-dev \
        libmount-dev libffi-dev libselinux1-dev libcap-ng-dev libattr1-dev \
        crossbuild-essential-arm64 gcc-10-aarch64-linux-gnu nettle-dev ninja-build

RUN apt update && \
    DEBIAN_FRONTEND=noninteractive \
    apt install -y --no-install-recommends \
        libcap-ng-dev:arm64 \
        libpixman-1-dev:arm64 \
        libglib2.0-dev:arm64 \
        libbz2-dev:arm64 \
        liblzo2-dev:arm64 \
        librdmacm-dev:arm64 \
        libsnappy-dev:arm64 \
        libxen-dev:arm64

RUN wget -q -O "qemu-6.0.0.tar.xz" https://download.qemu.org/qemu-6.0.0.tar.xz && \
  echo "87bc1a471ca24b97e7005711066007d443423d19aacda3d442558ae032fa30b9  qemu-6.0.0.tar.xz" | \
  sha256sum --check --status || (echo "qemu-6.0.0.tar.xz checksum verification failed!" && exit 1) && \
  tar -xf qemu-6.0.0.tar.xz

# --without-default-devices
 # --static \
RUN mkdir build && \
  cd build && \
  ../qemu-6.0.0/configure \
  --cross-prefix=aarch64-linux-gnu- \
  --target-list=x86_64-softmmu \
  --audio-drv-list="" \
  --enable-virtfs \
  --enable-vhost-vsock \
  --disable-slirp \
  --disable-tcg \
  --disable-tcg-interpreter  \
  --disable-containers --disable-gtk \
  --disable-capstone \
  --disable-avx2 \
  --disable-avx512f \
  --disable-replication \
  --disable-parallels \
  --disable-sheepdog \
  --disable-vvfat \
  --disable-qed \
  --disable-vdi \
  --disable-qcow1 \
  --disable-dmg \
  --disable-cloop \
  --disable-bochs \
  --disable-bzip2 \
  --disable-guest-agent \
  --disable-numa \
  --disable-vnc \
  --disable-live-block-migration \
  --without-default-devices

RUN echo CONFIG_PARALLEL=y >> config-host.mak && \
echo CONFIG_VIRTIO=y >> config-host.mak && \
echo CONFIG_VIRTIO_SERIAL=y >> config-host.mak && \
echo CONFIG_VIRTIO_PCI=y >> config-host.mak && \
echo CONFIG_VIRTIO_RNG=y >> config-host.mak && \
echo CONFIG_VIRTIO_MMIO=y >> config-host.mak && \
echo CONFIG_VIRTIO_SCSI=y >> config-host.mak && \
echo CONFIG_VIRTIO_BLK=y >> config-host.mak && \
echo CONFIG_VIRTIO_9P=y >> config-host.mak && \
echo CONFIG_FSDEV_9P=y >> config-host.mak

RUN cd build && make -j$(nproc)
RUN cp /build/x86_64-softmmu/qemu-system-x86_64 vmrt-aarch64


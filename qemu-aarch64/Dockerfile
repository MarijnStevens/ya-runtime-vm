FROM debian:bullseye-slim

# Add the foreign architecture we want and install dependencies
RUN dpkg --add-architecture arm64
RUN apt update && \
    DEBIAN_FRONTEND=noninteractive \
        apt install -y --no-install-recommends \
        build-essential ca-certificates git make \
        python3 wget pkg-config  	libglib2.0-0 libglib2.0-dev libpixman-1-dev \
        libmount-dev libffi-dev libselinux1-dev libcap-ng-dev libattr1-dev \
        crossbuild-essential-arm64 gcc-10-aarch64-linux-gnu nettle-dev

RUN wget - O "qemu-5.1.0.tar.xz" https://download.qemu.org/qemu-5.1.0.tar.xz && \
  echo c9174eb5933d9eb5e61f541cd6d1184cd3118dfe4c5c4955bc1bdc4d390fa4e5 "qemu-5.1.0.tar.xz" | \
  shas256sum -C || (echo "qemu-5.1.0.tar.xz checksum verification failed!" && exit 1) && \
  tar xvJf qemu-5.1.0.tar.xz -C qemu

# --without-default-devices
RUN mkdir build && \
  cd build && \
  /qemu/configure \
  --cross-prefix=aarch64-linux-gnu- \
  --target-list=x86_64-softmmu \
  --static \
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

RUN echo CONFIG_PARALLEL=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_SERIAL=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_PCI=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_RNG=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_MMIO=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_SCSI=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_BLK=y >> build/config-host.mak
RUN echo CONFIG_VIRTIO_9P=y >> build/config-host.mak
RUN echo CONFIG_FSDEV_9P=y >> build/config-host.mak


RUN cd build && make x86_64-softmmu/all V=1 CFLAGS+="-Os -flto" -j4 LIBS+="-flto -lblkid -luuid -lpixman-1 -lutil"
#RUN cd build && make x86_64-softmmu/all V=1 LIBS+="-flto -lblkid -luuid -lpixman-1 -lutil" CONFIG_PARALLEL=y CONFIG_VIRTIO_SERIAL=y

#RUN cd build/x86_64-softmmu && -lmount -lblkid -luuid

RUN cp /build/x86_64-softmmu/qemu-system-x86_64 vmrt-aarch64


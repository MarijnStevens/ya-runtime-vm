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
        libssl-dev:arm64 \
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

COPY build.sh /build.sh

RUN ./build.sh aarch64

RUN cp /build/aarch64-softmmu/qemu-system-aarch64 vmrt-aarch64


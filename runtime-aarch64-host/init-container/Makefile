CC := musl-gcc

CFLAGS := -std=c11 -O2 -Wall -Wextra -Werror -fPIE -pie -Iinclude/

ifneq ($(DEBUG), "")
CFLAGS += -DNDEBUG
endif

SRC_DIR ?= src
TEST_DIR ?= tests

KERNEL_VER ?= 5.10.38-0-virt
KERNEL_URL ?= http://dl-cdn.alpinelinux.org/alpine/v3.13/main/aarch64/linux-virt-5.10.38-r0.apk
KERNEL_SHA256 ?= 4b6d8cee15429c4b901f32d7de737766c18b8460301bb1872794047096ade85c

UNPACKED_DIR := unpacked_kernel

# Below are the steps performed by this Makefile:
# - download the kernel apk package
# - verify checksum of the downloaded package
# - extract the contents of the downloaded package
# - copy vmlinuz-virt from the extracted package
# - create a build directory for initramfs
# - copy required kernel modules from the extracted package to initramfs build directory
# - bundle initramfs.cpio.gz
# - compile init (https://en.wikipedia.org/wiki/Init) from sources

.DEFAULT_GOAL = all
.PHONY: all
all: vmlinuz-virt initramfs.cpio.gz

%.o: %.c
	$(CC) $(CFLAGS) -o $@ -c $^

init: $(addprefix $(SRC_DIR)/,init.o communication.o process_bookkeeping.o cyclic_buffer.o)
	$(CC) $(CFLAGS) -static -o $@ $^
	@# default musl libs on some distros have debug symbols, lets strip them (and everythig else)
	strip $@

$(UNPACKED_DIR):
	wget -q -O "unverified" $(KERNEL_URL)
	echo $(KERNEL_SHA256) "unverified" | sha256sum -c || (echo "Kernel apk checksum verification failed!" && exit 1)
	$(RM) -rf $(UNPACKED_DIR)
	mkdir $(UNPACKED_DIR)
	tar --warning=no-unknown-keyword -C $(UNPACKED_DIR) -vxzf unverified >/dev/null
	$(RM) unverified

vmlinuz-virt: $(UNPACKED_DIR)
	cp $(UNPACKED_DIR)/boot/vmlinuz-virt .

initramfs.cpio.gz: init $(UNPACKED_DIR)
	$(RM) -rf initramfs
	mkdir initramfs
	cp $< initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/virtio/virtio.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/virtio/virtio_ring.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/virtio/virtio_pci.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/char/hw_random/rng-core.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/char/hw_random/virtio-rng.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/char/virtio_console.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/drivers/block/virtio_blk.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/fs/squashfs/squashfs.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/fs/overlayfs/overlay.ko initramfs
	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/fs/fscache/fscache.ko initramfs
#	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/net/9p/9pnet.ko initramfs
#	cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/net/9p/9pnet_virtio.ko initramfs
# cp $(UNPACKED_DIR)/lib/modules/$(KERNEL_VER)/kernel/fs/9p/9p.ko
	cp init initramfs

	cd initramfs && find . | cpio --quiet -o -H newc -R 0:0 | gzip -9 > ../$@
	$(RM) -rf initramfs

TESTS_NAMES := cyclic_buffer
TESTS := $(addprefix $(TEST_DIR)/,$(TESTS_NAMES))

$(TESTS): %: %.o $(addprefix $(SRC_DIR)/,cyclic_buffer.o)
	$(CC) $(CFLAGS) -static -o $@ $^

.PHONY: test
test: $(TESTS)
	for t in $(TESTS) ; do \
		$$t ; \
	done

.PHONY: clean
clean:
	$(RM) init $(SRC_DIR)/*.o $(TEST_DIR)/*.o $(TESTS)
	$(RM) vmlinuz-virt initramfs.cpio.gz

.PHONY: distclean
distclean:
	$(RM) -rf $(UNPACKED_DIR)

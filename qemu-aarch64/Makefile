all: vmrt

vmrt: Dockerfile
	docker build -t build-qemu .
	$(SHELL) copy_img build-qemu vmrt-aarch64 .

.PHONY: all


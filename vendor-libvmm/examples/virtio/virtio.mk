#
# Copyright 2024, UNSW
#
# SPDX-License-Identifier: BSD-2-Clause
#
PYTHON ?= python3

LIBVMM_DOWNLOADS := https://trustworthy.systems/Downloads/libvmm/images/

LIBVMM_TOOLS := $(LIBVMM)/tools
MICROKIT_TOOL ?= $(MICROKIT_SDK)/bin/microkit
SDDF_INCLUDE := $(SDDF)/include/sddf
UTIL := $(SDDF)/util

TIMER_DRIVER := $(SDDF)/drivers/timer/$(TIMER_DRIVER)
SERIAL_DRIVER := $(SDDF)/drivers/serial/$(UART_DRIVER)
BLK_DRIVER := $(SDDF)/drivers/blk/$(BLK_DRIVER)
ETH_DRIVER := $(SDDF)/drivers/network/$(ETH_DRIVER)
SERIAL_COMPONENTS := $(SDDF)/serial/components
BLK_COMPONENTS := $(SDDF)/blk/components
NET_COMPONENTS := $(SDDF)/network/components

BOARD_DIR := $(MICROKIT_SDK)/board/$(MICROKIT_BOARD)/$(MICROKIT_CONFIG)
CLIENT_VM := $(VIRTIO_EXAMPLE)/client_vm
CLIENT_DTB := client_vm/vm.dtb
METAPROGRAM := $(VIRTIO_EXAMPLE)/meta.py

SDDF_CUSTOM_LIBC := 1

TOOLCHAIN ?= clang
SUPPORTED_BOARDS := \
	qemu_virt_aarch64 \
	maaxboard

SYSTEM_FILE := virtio.system
IMAGE_FILE := loader.img
REPORT_FILE := report.txt

include ${SDDF}/tools/make/board/common.mk

CLIENT_VM_USERLEVEL_INIT := blk_client_init net_client_init
CLIENT_VM_USERLEVEL_HOME := $(LIBVMM_TOOLS)/linux/blk/blk_integration_tests.sh $(LIBVMM_TOOLS)/linux/blk/blk_bench.sh

vpath %.c $(SDDF) $(LIBVMM) $(VIRTIO_EXAMPLE)

CFLAGS += \
	  -Wall \
	  -Wno-unused-function \
	  -DBOARD_$(MICROKIT_BOARD) \
	  -I$(BOARD_DIR)/include \
	  -I$(SDDF)/include \
	  -I$(SDDF)/include/microkit \
	  -I$(LIBVMM)/include \
	  -I$(VIRTIO_EXAMPLE)/include

LDFLAGS := -L$(BOARD_DIR)/lib
LIBS := --start-group -lmicrokit -Tmicrokit.ld libsddf_util_debug.a --end-group

include $(SDDF)/util/util.mk
ifeq ($(MICROKIT_BOARD), maaxboard)
include ${SDDF}/drivers/timer/${TIMER_DRIV_DIR}/timer_driver.mk
endif
include $(SERIAL_COMPONENTS)/serial_components.mk
include ${SDDF}/drivers/serial/${UART_DRIV_DIR}/serial_driver.mk
include ${SDDF}/drivers/network/${NET_DRIV_DIR}/eth_driver.mk
include ${SDDF}/drivers/blk/${BLK_DRIV_DIR}/blk_driver.mk
include $(BLK_COMPONENTS)/blk_components.mk
include $(NET_COMPONENTS)/network_components.mk
include $(LIBVMM)/vmm.mk
include $(LIBVMM_TOOLS)/linux/uio/uio.mk
include $(LIBVMM_TOOLS)/linux/blk/blk_init.mk
include $(LIBVMM_TOOLS)/linux/net/net_init.mk

IMAGES := client_vmm.elf timer_driver.elf blk_driver.elf blk_virt.elf serial_driver.elf serial_virt_tx.elf serial_virt_rx.elf \
	network_virt_rx.elf network_virt_tx.elf eth_driver.elf network_copy.elf

CHECK_FLAGS_BOARD_MD5 := .board_cflags-$(shell echo -- $(CFLAGS) $(BOARD) $(MICROKIT_CONFIG) | shasum | sed 's/ *-//')

$(CHECK_FLAGS_BOARD_MD5):
	-rm -f .board_cflags-*
	touch $@

all: ${IMAGE_FILE}

-include vmm.d

$(IMAGES): libsddf_util_debug.a

$(SYSTEM_FILE): $(METAPROGRAM) $(IMAGES) $(DTB) $(CLIENT_DTB)
	PYTHONPATH=${SDDF}/tools/meta:$$PYTHONPATH $(PYTHON) $(METAPROGRAM) --sddf $(SDDF) --board $(MICROKIT_BOARD) --dtb $(DTB) --client-dtb $(CLIENT_DTB) --output . --sdf $(SYSTEM_FILE) $(PARTITION_ARG)

ifeq ($(MICROKIT_BOARD), maaxboard)
	$(OBJCOPY) --update-section .device_resources=timer_driver_device_resources.data timer_driver.elf
	$(OBJCOPY) --update-section .timer_client_config=timer_client_blk_driver.data blk_driver.elf
endif
	$(OBJCOPY) --update-section .device_resources=blk_driver_device_resources.data blk_driver.elf
	$(OBJCOPY) --update-section .blk_driver_config=blk_driver.data blk_driver.elf
	$(OBJCOPY) --update-section .blk_virt_config=blk_virt.data blk_virt.elf
	$(OBJCOPY) --update-section .blk_client_config=blk_client_CLIENT_VMM.data client_vmm.elf
	$(OBJCOPY) --update-section .device_resources=serial_driver_device_resources.data serial_driver.elf
	$(OBJCOPY) --update-section .serial_driver_config=serial_driver_config.data serial_driver.elf
	$(OBJCOPY) --update-section .serial_virt_rx_config=serial_virt_rx.data serial_virt_rx.elf
	$(OBJCOPY) --update-section .serial_virt_tx_config=serial_virt_tx.data serial_virt_tx.elf
	$(OBJCOPY) --update-section .serial_client_config=serial_client_CLIENT_VMM.data client_vmm.elf
	$(OBJCOPY) --update-section .vmm_config=vmm_CLIENT_VMM.data client_vmm.elf
	$(OBJCOPY) --update-section .device_resources=eth_driver_device_resources.data eth_driver.elf
	$(OBJCOPY) --update-section .net_driver_config=net_driver.data eth_driver.elf
	$(OBJCOPY) --update-section .net_virt_rx_config=net_virt_rx.data network_virt_rx.elf
	$(OBJCOPY) --update-section .net_virt_tx_config=net_virt_tx.data network_virt_tx.elf
	$(OBJCOPY) --update-section .net_copy_config=net_copy_client0_net_copier.data network_copy.elf network_copy.elf
	$(OBJCOPY) --update-section .net_client_config=net_client_CLIENT_VMM.data client_vmm.elf

$(IMAGE_FILE) $(REPORT_FILE): $(IMAGES) $(SYSTEM_FILE)
	$(MICROKIT_TOOL) $(SYSTEM_FILE) --search-path $(BUILD_DIR) --board $(MICROKIT_BOARD) \
		--config $(MICROKIT_CONFIG) -o $(IMAGE_FILE) -r $(REPORT_FILE)

.PHONY: client_vm
client_vm:
	mkdir -p client_vm

${LINUX}:
	curl -L ${LIBVMM_DOWNLOADS}/$(LINUX).tar.gz -o $(LINUX).tar.gz
	mkdir -p linux_download_dir
	tar -xf $@.tar.gz -C linux_download_dir
	cp linux_download_dir/${LINUX}/linux ${LINUX}

${INITRD}:
	curl -L ${LIBVMM_DOWNLOADS}/$(INITRD).tar.gz -o $(INITRD).tar.gz
	mkdir -p initrd_download_dir
	tar xf $@.tar.gz -C initrd_download_dir
	cp initrd_download_dir/${INITRD}/rootfs.cpio.gz ${INITRD}

# client_vm/rootfs.cpio.gz was previously keyed only on ${INITRD}'s mtime,
# not its resolved path — an INITRD path switch (stock example -> a real
# product rootfs, or vice versa) with the new file's mtime older than the
# last-built target left Make considering it up to date, silently reusing
# the WRONG rootfs. Hit for real 2026-08-26 building os-mediakit's G3 (had
# to manually rm -rf client_vm/ to force a rebuild). Ported from
# project-totebox's own identical fix to this same shared example (their
# 2026-08-02 incident, same root cause) — adopting their proven guard
# rather than re-deriving it or continuing to work around it manually.
client_vm/.last_initrd_path: FORCE |client_vm
	@echo "${INITRD}" > client_vm/.last_initrd_path.tmp
	@cmp -s client_vm/.last_initrd_path.tmp client_vm/.last_initrd_path 2>/dev/null \
		&& rm -f client_vm/.last_initrd_path.tmp \
		|| mv client_vm/.last_initrd_path.tmp client_vm/.last_initrd_path

.PHONY: FORCE
FORCE:

client_vm/rootfs.cpio.gz: ${INITRD} client_vm/.last_initrd_path \
	$(CLIENT_VM_USERLEVEL_INIT) $(CLIENT_VM_USERLEVEL_HOME) |client_vm
	$(LIBVMM)/tools/packrootfs ${INITRD} \
		client_vm/rootfs_staging -o $@ \
		--startup $(CLIENT_VM_USERLEVEL_INIT) \
		--home $(CLIENT_VM_USERLEVEL_HOME)

blk_storage:
	$(SDDF)/tools/mkvirtdisk $@ $(BLK_NUM_PART) $(BLK_SIZE) $(BLK_MEM) GPT

# FOUNDRY_EXTRA_BOOTARGS substitution — see linux.dts's own
# @@FOUNDRY_EXTRA_BOOTARGS@@ comment for the full story. Ported from
# project-totebox's identical fix to this same shared example. Empty by
# default — a build with no override behaves exactly as before this fix.
FOUNDRY_EXTRA_BOOTARGS ?=

# Same staleness-guard pattern as client_vm/.last_initrd_path above: vm.dts's
# real file dependencies (linux.dts, the GIC overlay) don't change when only
# FOUNDRY_EXTRA_BOOTARGS changes between invocations, so without this Make
# would consider an already-built vm.dts up to date and silently reuse the
# previous build's bootargs.
client_vm/.last_bootargs: FORCE |client_vm
	@echo "${FOUNDRY_EXTRA_BOOTARGS}" > client_vm/.last_bootargs.tmp
	@cmp -s client_vm/.last_bootargs.tmp client_vm/.last_bootargs 2>/dev/null \
		&& rm -f client_vm/.last_bootargs.tmp \
		|| mv client_vm/.last_bootargs.tmp client_vm/.last_bootargs

client_vm/vm.dts: $(CLIENT_VM)/linux.dts $(CLIENT_VM)/$(GIC_DT_OVERLAY) \
	client_vm/.last_bootargs $(CHECK_FLAGS_BOARD_MD5) |client_vm
	$(LIBVMM)/tools/dtscat $(CLIENT_VM)/linux.dts $(CLIENT_VM)/$(GIC_DT_OVERLAY) > $@
	sed -i 's|@@FOUNDRY_EXTRA_BOOTARGS@@|$(FOUNDRY_EXTRA_BOOTARGS)|' $@

client_vm/vm.dtb: client_vm/vm.dts
	$(DTC) -q -I dts -O dtb $< > $@

client_vm/vmm.o: $(VIRTIO_EXAMPLE)/client_vmm.c $(CHECK_FLAGS_BOARD_MD5) |client_vm
	$(CC) $(CFLAGS) -c -o $@ $<

client_vm/images.o: $(LIBVMM)/tools/package_guest_images.S $(CHECK_FLAGS_BOARD_MD5) \
	${LINUX} client_vm/vm.dtb client_vm/rootfs.cpio.gz
	$(CC) -c -g3 -x assembler-with-cpp \
					-DGUEST_KERNEL_IMAGE_PATH=\"${LINUX}\" \
					-DGUEST_DTB_IMAGE_PATH=\"client_vm/vm.dtb\" \
					-DGUEST_INITRD_IMAGE_PATH=\"client_vm/rootfs.cpio.gz\" \
					-target $(TARGET) \
					$(LIBVMM)/tools/package_guest_images.S -o $@

client_vmm.elf: client_vm/vmm.o client_vm/images.o libvmm.a |client_vm
	$(LD) $(LDFLAGS) $^ $(LIBS) -o $@

# Stop make from deleting intermediate files
.PRECIOUS: client_vm client_vm/vm.dts client_vm/vm.dtb \
	client_vm/rootfs.cpio.gz client_vm/images.o client_vm/vmm.o

qemu: $(IMAGE_FILE) blk_storage
	[ ${MICROKIT_BOARD} = qemu_virt_aarch64 ]
	$(QEMU) -machine virt,virtualization=on,secure=off \
			-cpu cortex-a53 \
			-serial mon:stdio \
			-device loader,file=$(IMAGE_FILE),addr=0x70000000,cpu-num=0 \
			-m size=2G \
			-nographic \
			-global virtio-mmio.force-legacy=false \
			-drive file=blk_storage,format=raw,if=none,id=hd \
			$(QEMU_BLK_ARGS) \
			$(QEMU_NET_ARGS) \
			-netdev user,id=netdev0,hostfwd=tcp::1236-:1236,hostfwd=tcp::1237-:1237,hostfwd=udp::1235-:1235 \

clean::
	$(RM) -f *.elf .depend* $
	find . -name \*.[do] -type f |xargs --no-run-if-empty rm

clobber:: clean
	rm -f *.a
	rm -f $(IMAGE_FILE) $(REPORT_FILE)

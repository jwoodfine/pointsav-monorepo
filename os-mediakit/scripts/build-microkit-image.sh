#!/usr/bin/env bash
# SPDX-License-Identifier: FSL-1.1-ALv2
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# build-microkit-image.sh — G1 (Boot): guarded wrapper around vendor-libvmm's
# Microkit `make ... qemu` link step for os-mediakit.
#
# Ported from project-totebox's os-totebox/scripts/build-microkit-image.sh
# (2026-08-25), including its resource-pressure preflight guard verbatim —
# that guard exists because project-totebox found via filesystem-mtime
# forensics that this exact kind of build (C/C++ compile + link of
# blk_virt/net_virt/serial_virt/client_vmm, culminating in loader.img) was
# actively running at the moment of a real host crash. Not a hypothetical
# risk; reuse the same thresholds, don't invent a third variant.
#
# G1 scope only: this proves the Microkit+libvmm+TCG pipeline itself boots
# to a real login prompt on this host, using vendor-libvmm's own bundled
# example Linux kernel + initrd (auto-downloaded by virtio.mk if not already
# present — unmodified, has nothing to do with app-mediakit-knowledge).
# Porting a real os-mediakit guest rootfs with the actual wiki binary inside
# is G2.5+, not this script — do not add an INITRD override here until that
# phase starts; keep G1 and G2.5 as separately-verifiable gates per the
# BRIEF's own gate-ladder sequencing.
#
# Usage: run from os-mediakit/ (same convention as os-totebox):
#   bash scripts/build-microkit-image.sh
set -euo pipefail

BUILD_DIR="build-os-mediakit"
MICROKIT_BOARD="qemu_virt_aarch64"
MICROKIT_SDK="${MICROKIT_SDK:-/opt/microkit-sdk-2.2.0}"
LIBVMM_VIRTIO_DIR="../vendor-libvmm/examples/virtio"

[ -d "${MICROKIT_SDK}" ] || {
    echo "error: MICROKIT_SDK not found at ${MICROKIT_SDK}" >&2
    exit 1
}
[ -d "${LIBVMM_VIRTIO_DIR}" ] || {
    echo "error: ${LIBVMM_VIRTIO_DIR} not found — run from os-mediakit/" >&2
    exit 1
}

# ── Preflight resource guard — identical thresholds/logic to
# os-totebox's build-microkit-image.sh (itself sourced from
# bin/resource-log.sh's pressure-snapshot trigger). Hard-abort, not
# warn-and-continue: this host is shared across concurrent Totebox
# sessions and a live inference service, and a heavy Microkit/libvmm
# make build has a real, evidenced crash history when run under
# pressure.
_mem_total_kb=$(awk '/^MemTotal:/{print $2}' /proc/meminfo)
_mem_avail_kb=$(awk '/^MemAvailable:/{print $2}' /proc/meminfo)
_mem_used_pct=$(awk "BEGIN{printf \"%.1f\", (${_mem_total_kb}-${_mem_avail_kb})/${_mem_total_kb}*100}")
_psi_mem_some=$(awk '/^some/ {for(i=1;i<=NF;i++) if($i~/^avg10=/) {sub(/avg10=/,"",$i); print $i; exit}}' \
    /sys/fs/cgroup/memory.pressure 2>/dev/null || echo "0.00")
read -r _load_1 _ _ _ < /proc/loadavg
_nproc_count=$(nproc 2>/dev/null || echo 1)
if awk "BEGIN{exit !(${_mem_used_pct} > 85 || ${_psi_mem_some} > 10 || ${_load_1} >= ${_nproc_count})}"; then
    echo "error: refusing to start the Microkit/libvmm make build — host already under pressure" >&2
    echo "  mem_used_pct=${_mem_used_pct}% psi_mem_some_avg10=${_psi_mem_some} load_1=${_load_1} nproc=${_nproc_count}" >&2
    echo "  (thresholds match bin/resource-log.sh's own pressure-snapshot trigger)" >&2
    echo "  wait for load/memory to settle and rerun -- this is a hard gate, not a warning" >&2
    exit 1
fi
echo "  preflight OK: mem_used_pct=${_mem_used_pct}% psi=${_psi_mem_some} load_1=${_load_1}/${_nproc_count}"

echo "  building Microkit/libvmm image (BUILD_DIR=${BUILD_DIR}, stock example LINUX/INITRD)..."
exec make -C "${LIBVMM_VIRTIO_DIR}" \
    MICROKIT_BOARD="${MICROKIT_BOARD}" \
    MICROKIT_SDK="${MICROKIT_SDK}" \
    BUILD_DIR="${BUILD_DIR}" \
    qemu

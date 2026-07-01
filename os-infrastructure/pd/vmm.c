/*
 * os-infrastructure — CAmkES VMM protection domain (placeholder)
 *
 * This is the placeholder for the CAmkES Virtual Machine Monitor PD.
 * When complete it will bring up a Linux (Debian 12) guest VM inside
 * the seL4 microkernel, hosting WireGuard mesh join + service-vm-fleet.
 *
 * Phase gate: Phase S4 (Genesis Protocol) on the network side;
 * moonshot-sel4-vmm for the full VMM implementation.
 */
#include <microkit.h>

void init(void)
{
    microkit_dbg_puts("os-infrastructure: seL4 VMM protection domain active\n");
    microkit_dbg_puts("os-infrastructure: placeholder — CAmkES VMM pending\n");
}

void notified(microkit_channel ch)
{
    (void)ch;
}

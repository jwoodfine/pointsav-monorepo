// LEGACY / ORPHANED — this bare-metal Multiboot2 kernel predates the seL4+Microkit
// pipeline and is NOT consumed by `moonshot-toolkit build` (which builds from
// system-spec.toml + pd/vmm.c instead — see CLAUDE.md "Build path"). GRUB2 boots
// seL4 directly via the Microkit-assembled image; this file never runs in that chain.
// Kept per CLAUDE.md "Existing scaffold" (do not delete). EAPOL-monitor-mode retired
// 2026-07-09 (operator decision: Genesis Protocol, not EAPOL) — real Genesis Protocol
// work belongs in pd/vmm.c, not here. This file is now an inert placeholder.
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    ".section .multiboot",
    ".align 8",
    "header_start:",
    ".long 0xe85250d6",
    ".long 0",
    ".long header_end - header_start",
    ".long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))",
    ".short 0", ".short 0", ".long 8",
    "header_end:",
    ".section .text",
    ".global _start",
    "_start:",
    "lea rsp, [rip + stack_top]",
    "call rust_main",
    "halt_loop:",
    "hlt",
    "jmp halt_loop",
    ".section .bss",
    ".align 16",
    "stack_bottom:",
    ".skip 16384",
    "stack_top:"
);

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

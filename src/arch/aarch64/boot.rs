// src/arch/aarch64/boot.rs
use core::arch::global_asm;

// Define the entry point using inline assembly
global_asm!(
    r#"
    .section .text._start
    .global _start
    .type _start, %function

_start:
    /* 1. 设置栈指针（SP_EL0） */
    mov x0, #0x80000        /* 栈顶在 0x80000 + 64KB = 0x90000 */
    add x0, x0, #0x10000    /* 64KB 栈空间 */
    mov sp, x0

    /* 2. 确保在 EL1（操作系统级别） */
    mrs x0, CurrentEL
    cmp x0, #0x4            /* EL1 << 2 */
    b.eq 2f                 /* 已在 EL1，跳过降级 */

    /* 如果在 EL2，降级到 EL1 */
    mov x0, #(0x3c0 | 0x1)  /* HCR_EL2: RW=1 (EL1 is AArch64), enable VM */
    msr hcr_el2, x0
    isb

    /* 设置 SCTLR_EL1 基础值（稍后在 Rust 中完善） */
    mov x0, #0x30d0         /* ICache/DCache/MMU disabled, but aligned */
    msr sctlr_el1, x0
    isb

    /* 从 EL2 跳转到 EL1 */
    mov x0, #0x1            /* SPSR_EL2: D/A/I disabled, EL1h mode */
    msr spsr_el2, x0
    adr x0, 2f
    msr elr_el2, x0
    eret

2:
    /* 3. 清零 BSS 段 */
    ldr x0, =__bss_start
    ldr x1, =__bss_end
    mov x2, #0
1:
    cmp x0, x1
    bge 3f
    str x2, [x0], #8
    b 1b

3:
    /* 4. 跳转到 Rust main */
    bl rust_main

    /* 5. 死循环（不应返回） */
halt:
    wfe
    b halt

    .section .bss.stack
    .align 12
    .space 0x10000  /* 64KB 栈空间 */
    "#
);

// External symbols that need to be defined in the linker script
unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

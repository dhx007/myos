// src/arch/riscv64/boot.rs
use core::arch::global_asm;

// Define the entry point using inline assembly for RISC-V 64
global_asm!(
    r#"
    .section .text._start
    .global _start
    .type _start, @function

_start:
    /* 1. 设置栈指针 */
    la sp, _stack_end

    /* 2. 清零 BSS 段 */
    la a0, __bss_start
    la a1, __bss_end
    li a2, 0
1:
    bge a0, a1, 2f
    sd a2, 0(a0)
    addi a0, a0, 8
    j 1b

2:
    /* 3. 跳转到 Rust main */
    call rust_main

    /* 4. 死循环（不应返回） */
halt:
    wfi
    j halt

    .section .bss.stack
    .align 12
_stack_start:
    .space 0x10000  /* 64KB 栈空间 */
_stack_end:
    "#
);

// External symbols that need to be defined in the linker script
unsafe extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

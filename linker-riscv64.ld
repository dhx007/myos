/* RISC-V 64 裸机内核链接脚本 */
ENTRY(_start)

SECTIONS
{
    /* QEMU virt 机器通常从 0x80000000 加载内核 */
    . = 0x80000000;

    .text : {
        KEEP(*(.text._start))
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.debug*)
    }
}

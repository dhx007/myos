/* Raspberry Pi 3/4 裸机内核加载地址 */
ENTRY(_start)

SECTIONS
{
    /* Pi 3/4 的 GPU 会把 kernel8.img 加载到 0x80000 */
    . = 0x80000;

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

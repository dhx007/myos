// src/drivers/aarch64/uart.rs
// ARM64 UART 驱动实现
// 针对 Raspberry Pi 3/4 的 PL011 UART

pub fn init() {
    unsafe {
        const GPIO_BASE: u64 = 0x3F20_0000;
        const UART0_BASE: u64 = 0x3F20_1000;

        // 1. 配置 GPIO 14/15 为 ALT5 (UART)
        let gpfsel1 = GPIO_BASE + 0x04;
        let val = core::ptr::read_volatile(gpfsel1 as *const u32);
        let val = (val & !0b111111_00000000000000000000000000) |
                  (0b010 << 12) | // GPIO14
                  (0b010 << 15); // ← 修正：GPIO15 是 bit 15-17，不是 16！
        core::ptr::write_volatile(gpfsel1 as *mut u32, val);

        // 2. 禁用上拉/下拉
        core::ptr::write_volatile((GPIO_BASE + 0x94) as *mut u32, 0);
        core::arch::asm!("nop; nop; nop; nop");
        core::ptr::write_volatile((GPIO_BASE + 0x98) as *mut u32, (1 << 14) | (1 << 15));
        core::arch::asm!("nop; nop; nop; nop");
        core::ptr::write_volatile((GPIO_BASE + 0x98) as *mut u32, 0);

        // 3. 禁用 UART
        core::ptr::write_volatile((UART0_BASE + 0x30) as *mut u32, 0);

        // 4. 清除中断
        core::ptr::write_volatile((UART0_BASE + 0x44) as *mut u32, 0x7FF);

        // 5. 设置波特率 (115200 @ 500MHz)
        core::ptr::write_volatile((UART0_BASE + 0x24) as *mut u32, 271); // IBRD
        core::ptr::write_volatile((UART0_BASE + 0x28) as *mut u32, 0); // FBRD

        // 6. 8-bit, FIFO
        core::ptr::write_volatile((UART0_BASE + 0x2C) as *mut u32, (1 << 5) | (1 << 6));

        // 7. 使能 UART
        core::ptr::write_volatile(
            (UART0_BASE + 0x30) as *mut u32,
            (1 << 0) | (1 << 8) | (1 << 9),
        );
    }
}

// 发送一个字符
pub fn putc(c: u8) {
    unsafe {
        // PL011 UART DR 寄存器（Raspberry Pi 3/4）
        const UART0_DR: *mut u32 = 0x3F20_1000 as *mut u32;
        // 等待发送缓冲区空
        const UART0_FR: *const u32 = 0x3F20_1018 as *const u32;
        while core::ptr::read_volatile(UART0_FR) & (1 << 5) != 0 {}
        core::ptr::write_volatile(UART0_DR, c as u32);
    }
}

// 接收一个字符 (阻塞)
pub fn getc() -> u8 {
    unsafe {
        const UART0_DR: *const u32 = 0x3F20_1000 as *const u32;
        const UART0_FR: *const u32 = 0x3F20_1018 as *const u32;

        // 等待接收缓冲区有数据
        while core::ptr::read_volatile(UART0_FR) & (1 << 4) != 0 {}

        core::ptr::read_volatile(UART0_DR) as u8
    }
}

// 检查是否有数据可读
pub fn has_data() -> bool {
    unsafe {
        const UART0_FR: *const u32 = 0x3F20_1018 as *const u32;
        core::ptr::read_volatile(UART0_FR) & (1 << 4) == 0
    }
}

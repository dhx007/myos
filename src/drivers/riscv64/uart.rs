// src/drivers/riscv64/uart.rs
// RISC-V 64 UART 驱动实现
// 针对 QEMU virt 机器的 NS16550A 兼容 UART

pub fn init() {
    unsafe {
        // QEMU virt 机器的 UART 基地址
        const UART_BASE: u64 = 0x10000000;

        // NS16550A 寄存器偏移
        const UART_IER: u64 = 0x01; // 中断使能寄存器
        const UART_FCR: u64 = 0x02; // FIFO 控制寄存器 (只写)
        const UART_LCR: u64 = 0x03; // 线路控制寄存器
        const UART_MCR: u64 = 0x04; // Modem 控制寄存器

        // 禁用中断
        core::ptr::write_volatile((UART_BASE + UART_IER) as *mut u8, 0x00);

        // 设置波特率 (115200)
        // 对于 QEMU virt 机器，通常不需要设置波特率，使用默认值

        // 设置线路控制寄存器: 8位数据，无奇偶校验，1位停止位
        core::ptr::write_volatile((UART_BASE + UART_LCR) as *mut u8, 0x03);

        // 启用 FIFO 并清除
        core::ptr::write_volatile((UART_BASE + UART_FCR) as *mut u8, 0x07);

        // 启用 DTR, RTS, 和 OUT2
        core::ptr::write_volatile((UART_BASE + UART_MCR) as *mut u8, 0x0B);
    }
}

// 发送一个字符
pub fn putc(c: u8) {
    unsafe {
        const UART_BASE: u64 = 0x10000000;
        const UART_LSR: u64 = 0x05;
        const UART_THR: u64 = 0x00;

        // 等待发送缓冲区为空
        while core::ptr::read_volatile((UART_BASE + UART_LSR) as *const u8) & 0x20 == 0 {}

        // 发送字符
        core::ptr::write_volatile((UART_BASE + UART_THR) as *mut u8, c);
    }
}

// 接收一个字符 (阻塞)
pub fn getc() -> u8 {
    unsafe {
        const UART_BASE: u64 = 0x10000000;
        const UART_LSR: u64 = 0x05;
        const UART_RBR: u64 = 0x00;

        // 等待数据就绪
        while core::ptr::read_volatile((UART_BASE + UART_LSR) as *const u8) & 0x01 == 0 {}

        // 读取字符
        core::ptr::read_volatile((UART_BASE + UART_RBR) as *const u8)
    }
}

// 检查是否有数据可读
pub fn has_data() -> bool {
    unsafe {
        const UART_BASE: u64 = 0x10000000;
        const UART_LSR: u64 = 0x05;

        core::ptr::read_volatile((UART_BASE + UART_LSR) as *const u8) & 0x01 != 0
    }
}

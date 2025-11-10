// src/arch/riscv64/timer.rs
const CLINT_BASE: u64 = 0x2000000; // QEMU virt CLINT base
const MTIME: u64 = CLINT_BASE + 0xBFF8; // 64-bit real-time counter
const MTIMECMP: u64 = CLINT_BASE + 0x4000; // per-hart timer compare (hart 0)

/// 设置定时器中断（纳秒）
pub fn set_timer(timeout_ns: u64) {
    unsafe {
        // 1. 读取当前时间
        let current_time = core::ptr::read_volatile(MTIME as *const u64);

        // 2. 计算目标时间（QEMU virt 时钟频率 = 10 MHz）
        // 1 tick = 100 ns → timeout_ns / 100
        let ticks = timeout_ns / 100;
        let target_time = current_time + ticks;

        // 3. 写入 mtimecmp
        core::ptr::write_volatile(MTIMECMP as *mut u64, target_time);
    }
}

/// 获取当前时间（纳秒）
pub fn get_time_ns() -> u64 {
    unsafe {
        core::ptr::read_volatile(MTIME as *const u64) * 100 // 1 tick = 100 ns
    }
}

/// 简单延时（忙等待）
pub fn delay_ns(ns: u64) {
    let start = get_time_ns();
    while get_time_ns() - start < ns {}
}

// src/arch/aarch64/timer.rs
use core::arch::asm;

/// 获取系统计数器频率（Hz）
fn get_counter_freq() -> u64 {
    let freq: u64;
    unsafe {
        asm!("mrs {}, cntfrq_el0", out(reg) freq);
    }
    freq
}

/// 设置定时器（纳秒）
pub fn set_timer(timeout_ns: u64) {
    let freq = get_counter_freq();
    let ticks = timeout_ns * freq / 1_000_000_000;

    // 读取当前计数器
    let current: u64;
    unsafe {
        asm!("mrs {}, cntpct_el0", out(reg) current);
    }

    // 设置比较值
    unsafe {
        asm!("msr cntp_cval_el0, {}", in(reg) current + ticks);
        asm!("msr cntp_ctl_el0, {}", in(reg) 1); // 使能定时器
    }
}

/// 延时（纳秒）
pub fn delay_ns(ns: u64) {
    let start = get_time_ns();
    while get_time_ns() - start < ns {}
}

/// 获取当前时间（纳秒）
pub fn get_time_ns() -> u64 {
    let cnt: u64;
    unsafe {
        asm!("mrs {}, cntpct_el0", out(reg) cnt);
    }
    let freq = get_counter_freq();
    cnt * 1_000_000_000 / freq
}

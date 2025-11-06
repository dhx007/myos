#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

// 架构特定的 UART 初始化函数
#[cfg(target_arch = "aarch64")]
pub fn init() {
    aarch64::uart::init();
}

#[cfg(target_arch = "riscv64")]
pub fn init() {
    riscv64::uart::init();
}

// 架构特定的控制台实现
#[cfg(target_arch = "aarch64")]
pub use aarch64::uart::putc;

#[cfg(target_arch = "riscv64")]
pub use riscv64::uart::putc;

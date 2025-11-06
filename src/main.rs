#![no_std]
#![no_main]

// 声明模块（只声明一次！）
mod arch;
mod drivers;

// 全局 panic handler（必须！）
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// 控制台（简化版，直接用 UART）
use core::fmt::Write;
struct Console;
impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            unsafe {
                // PL011 UART DR 寄存器（Raspberry Pi 3/4）
                const UART0_DR: *mut u32 = 0x3F20_1000 as *mut u32;
                // 等待发送缓冲区空
                const UART0_FR: *const u32 = 0x3F20_1018 as *const u32;
                while core::ptr::read_volatile(UART0_FR) & (1 << 5) != 0 {}
                core::ptr::write_volatile(UART0_DR, c as u32);
            }
        }
        Ok(())
    }
}

// 简化 print! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(Console, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// 主函数（由汇编调用）
#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // 初始化 UART（配置 GPIO 和 波特率）
    drivers::uart::init();

    print!("Hello from ARM64 OS!\n");

    loop {
        print!(".");
        for _ in 0..1000000 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

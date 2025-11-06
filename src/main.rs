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

// 架构特定的控制台实现
#[cfg(target_arch = "aarch64")]
mod console {
    use crate::drivers;
    use core::fmt::Write;

    pub struct Console;

    impl Write for Console {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.bytes() {
                drivers::putc(c);
            }
            Ok(())
        }
    }
}

#[cfg(target_arch = "riscv64")]
mod console {
    use crate::drivers;
    use core::fmt::Write;

    pub struct Console;

    impl Write for Console {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            for c in s.bytes() {
                drivers::putc(c);
            }
            Ok(())
        }
    }
}

// 简化 print! 宏
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(console::Console, $($arg)*);
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
    // 初始化 UART
    drivers::init();

    // 显示架构信息
    #[cfg(target_arch = "aarch64")]
    println!("Hello from ARM64 OS!");

    #[cfg(target_arch = "riscv64")]
    println!("Hello from RISC-V 64 OS!");

    loop {
        print!(".");
        for _ in 0..1000000 {
            unsafe {
                core::arch::asm!("nop");
            }
        }
    }
}

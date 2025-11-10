# MyOS

一个用 Rust 编写的最小化、多架构操作系统。该项目演示了裸机编程概念，支持 ARM64 (树莓派 3/4) 和 RISC-V 64 (QEMU virt 机器) 架构。

## 功能特性

- **多架构支持**: 支持 ARM64 和 RISC-V 64 位架构
- **裸机运行**: 无标准库 (`no_std`)，直接在硬件上运行
- **UART 控制台**: 通过 UART 提供基本的输入/输出功能用于调试
- **汇编引导代码**: 为每个架构定制的汇编引导序列
- **内存管理**: 基本的 BSS 段置零和栈初始化

## 支持的平台

- **ARM64**: 面向树莓派 3/4 (PL011 UART)
- **RISC-V 64**: 面向 QEMU virt 机器 (NS16550A 兼容 UART)

## 前置条件

要构建和运行此操作系统，您需要：

- Rust 工具链 (edition 2024)
- 交叉编译目标:
  - `aarch64-unknown-none`
  - `riscv64gc-unknown-none-elf`
- `cargo-binutils` 用于生成二进制文件
- QEMU 用于模拟 (用于测试)
- 对于 ARM64: 树莓派 3/4 硬件或兼容的模拟器

安装前置条件:

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装交叉编译目标
rustup target add aarch64-unknown-none riscv64gc-unknown-none-elf

# 安装 cargo-binutils
cargo install cargo-binutils
```

## 构建

该项目提供了一个构建脚本，可以为所有支持的架构进行编译:

```bash
# 运行构建脚本来为所有目标编译
./build-all.sh
```

这将生成:
- `kernel8-arm64.img` 用于 ARM64 平台
- `kernel8-riscv64.img` 用于 RISC-V 64 平台

### 手动构建

要为特定架构手动构建:

```bash
# 为 ARM64 构建
cargo build --target aarch64-unknown-none --release

# 为 RISC-V 64 构建
cargo build --target riscv64gc-unknown-none-elf --release
```

生成二进制镜像:

```bash
# 生成 ARM64 二进制文件
cargo objcopy --bin myos --target aarch64-unknown-none --release -- -O binary kernel8-arm64.img

# 生成 RISC-V 64 二进制文件
cargo objcopy --bin myos --target riscv64gc-unknown-none-elf --release -- -O binary kernel8-riscv64.img
```

## 运行

### QEMU (RISC-V 64)

```bash
qemu-system-riscv64 -machine virt -nographic -kernel kernel8-riscv64.img
```

### 树莓派 3/4 (ARM64)

1. 将 `kernel8-arm64.img` 复制到您的 SD 卡
2. 替换原始的 `kernel8.img` 或适当重命名
3. 将 SD 卡插入树莓派并启动

## 架构

系统组织如下:

- `src/main.rs`: 主入口点和通用功能
- `src/arch/`: 架构特定的引导代码
  - `aarch64/boot.rs`: ARM64 汇编引导序列
  - `riscv64/`: RISC-V 64 引导代码
- `src/drivers/`: 硬件驱动程序
  - `aarch64/uart.rs`: 用于 ARM64 的 PL011 UART 驱动程序
  - `riscv64/uart.rs`: 用于 RISC-V 的 NS16550A 兼容 UART 驱动程序

### 引导过程

1. 汇编代码初始化栈并确保 EL1/Supervisor 模式
2. BSS 段被置零
3. 控制权转移给 `rust_main()` 函数
4. 初始化 UART 用于控制台输出
5. 打印 "Hello from [Architecture] OS!" 消息
6. 操作系统进入无限循环，打印点号

### UART 控制台

操作系统通过 UART 提供基本的控制台功能:

- `print!` 和 `println!` 宏用于输出
- `putc()` 函数用于字符输出
- `getc()` 函数用于字符输入 (阻塞)
- `has_data()` 函数用于检查可用输入

## 配置

该项目在 `Cargo.toml` 中使用 `bootimage` crate 元数据来指定目标和链接脚本:

- ARM64 使用 `linker.ld`
- RISC-V 64 使用 `linker-riscv64.ld`

构建过程通过 `build.rs` 脚本自定义，该脚本根据目标架构选择适当的链接脚本。

## 自定义

要扩展此操作系统:

1. 为每个平台在 `src/drivers/` 目录中添加新的驱动程序
2. 在 `src/main.rs` 中实现新的系统功能
3. 在 `src/arch/` 目录中添加架构特定代码
4. 如果添加新段，请更新链接脚本

## 贡献

该项目作为裸机 Rust OS 开发的教育示例。欢迎贡献，特别是:

- 添加硬件驱动程序
- 内存管理改进
- 进程调度
- 文件系统实现


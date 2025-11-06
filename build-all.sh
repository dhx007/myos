#!/bin/bash

# Build script for myos - supports both ARM64 and RISC-V 64 architectures

set -e

echo "Building myos for all supported architectures..."

# Clean previous builds
echo "Cleaning previous builds..."
cargo clean

# Build for ARM64 (Raspberry Pi 3/4)
echo "Building for ARM64 (aarch64-unknown-none)..."
cargo build --target aarch64-unknown-none --release
cargo objcopy --bin myos --target aarch64-unknown-none --release -- -O binary kernel8-arm64.img
echo "ARM64 binary created: kernel8-arm64.img"

# Build for RISC-V 64 (QEMU virt machine)
echo "Building for RISC-V 64 (riscv64gc-unknown-none-elf)..."
cargo build --target riscv64gc-unknown-none-elf --release
cargo objcopy --bin myos --target riscv64gc-unknown-none-elf --release -- -O binary kernel8-riscv64.img
echo "RISC-V 64 binary created: kernel8-riscv64.img"

# Display file sizes
echo ""
echo "Build completed successfully!"
echo "Binary sizes:"
ls -la kernel8*.img

echo ""
echo "Usage:"
echo "  ARM64:    Use kernel8-arm64.img on Raspberry Pi 3/4"
echo "  RISC-V 64: Use kernel8-riscv64.img with QEMU virt machine"

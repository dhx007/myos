// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 获取目标架构
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // 根据目标架构选择链接脚本
    if target.contains("aarch64") {
        let link_script = PathBuf::from("linker.ld");
        std::fs::copy(&link_script, out_dir.join("linker.ld")).unwrap();
        println!("cargo:rustc-link-arg=-Tlinker.ld");
        println!("cargo:rerun-if-changed=linker.ld");
    } else if target.contains("riscv64") {
        let link_script = PathBuf::from("linker-riscv64.ld");
        std::fs::copy(&link_script, out_dir.join("linker-riscv64.ld")).unwrap();
        println!("cargo:rustc-link-arg=-Tlinker-riscv64.ld");
        println!("cargo:rerun-if-changed=linker-riscv64.ld");
    } else {
        panic!("Unsupported target architecture: {}", target);
    }
}

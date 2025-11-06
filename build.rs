// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    // 指定链接脚本
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let link_script = PathBuf::from("linker.ld");
    std::fs::copy(&link_script, out_dir.join("linker.ld")).unwrap();
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rerun-if-changed=linker.ld");
}

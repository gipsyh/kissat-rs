#![feature(exit_status_error)]
use cmake::Config;

fn main() -> Result<(), String> {
    giputils::build::git_submodule_update()?;
    println!("cargo:rerun-if-changed=./kissat");
    println!("cargo:rerun-if-changed=./CMakeLists.txt");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let mut cfg = Config::new(".");
    if target_os == "windows" && target_env == "gnu" {
        cfg.define("CMAKE_C_COMPILER", "x86_64-w64-mingw32-gcc");
        cfg.define("CMAKE_SYSTEM_NAME", "Windows");
    } else {
        cfg.define("CMAKE_C_COMPILER", "clang");
        cfg.define("CMAKE_C_FLAGS", "-flto");
    }
    cfg.define("CMAKE_BUILD_TYPE", "Release");
    let dst = cfg.build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=satif-kissat");
    Ok(())
}

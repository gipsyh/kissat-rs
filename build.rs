use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .expect("Failed to update submodules.");

    let src_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Environmental variable `CARGO_MANIFEST_DIR` not defined.".to_string())?;

    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(src_dir).display()
    );
    println!("cargo:rustc-link-lib=static=kissat");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}

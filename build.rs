#![feature(exit_status_error)]

use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
extern crate giputils;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .unwrap();
    println!("cargo:rerun-if-changed=./kissat");
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let kissat_src = PathBuf::from(src_dir).join("kissat");
    let kissat_out = PathBuf::from(out_dir).join("kissat");
    let num_jobs = env::var("NUM_JOBS").unwrap();
    if kissat_out.exists() {
        fs::remove_dir_all(&kissat_out).unwrap();
    }
    fs::create_dir(&kissat_out).unwrap();
    let overlay = giputils::mount::MountOverlay::new(&kissat_src, &kissat_out);
    Command::new("sh")
        .arg("configure")
        .current_dir(overlay.path())
        .status()
        .map_err(|e| e.to_string())?
        .exit_ok()
        .map_err(|e| e.to_string())?;
    Command::new("make")
        .arg(format!("-j{}", num_jobs))
        .current_dir(overlay.path())
        .status()
        .map_err(|e| e.to_string())?
        .exit_ok()
        .map_err(|e| e.to_string())?;
    println!(
        "cargo:rustc-link-search=native={}",
        kissat_out.join("build").display()
    );
    println!("cargo:rustc-link-lib=static=kissat");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}

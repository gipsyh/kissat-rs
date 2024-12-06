use std::env;
use std::env::temp_dir;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), String> {
    Command::new("git")
        .args(["submodule", "update", "--init"])
        .status()
        .expect("Failed to update submodules.");

    let src_dir = env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| "Environmental variable `CARGO_MANIFEST_DIR` not defined.".to_string())?;
    let out_dir = env::var("OUT_DIR")
        .map_err(|_| "Environmental variable `OUT_DIR` not defined.".to_string())?;
    println!("cargo:rerun-if-changed=./kissat");
    let kissat_src = PathBuf::from(&src_dir).join("kissat");
    let out_dir = PathBuf::from(out_dir);
    let kissat_temp = temp_dir().join("satif-kissat");
    Command::new("rm")
        .args(["-rf", kissat_temp.to_str().unwrap()])
        .status()
        .expect("Failed to remove old kissat build directory.");
    Command::new("cp")
        .args([
            "-r",
            kissat_src.to_str().unwrap(),
            kissat_temp.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to copy kissat source.");
    Command::new("sh")
        .arg("configure")
        .current_dir(&kissat_temp)
        .status()
        .expect("Failed to configure kissat.");
    Command::new("make")
        .current_dir(&kissat_temp)
        .status()
        .expect("Failed to build kissat.");
    let kissat_lib = kissat_temp.join("build").join("libkissat.a");
    Command::new("cp")
        .arg(kissat_lib)
        .arg(&out_dir)
        .status()
        .expect("Failed to copy kissat library.");
    Command::new("rm")
        .args(["-rf", kissat_temp.to_str().unwrap()])
        .status()
        .expect("Failed to remove old kissat build directory.");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=kissat");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    Ok(())
}

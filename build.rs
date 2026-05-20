use giputils::build::copy_build;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::{env, io};
extern crate giputils;

const KITTEN_SYMBOLS: &[&str] = &[
    "completely_backtrack_to_root_level",
    "kitten_assume",
    "kitten_binary",
    "kitten_clause",
    "kitten_clause_with_id_and_exception",
    "kitten_clear",
    "kitten_compute_clausal_core",
    "kitten_embedded",
    "kitten_failed",
    "kitten_fixed",
    "kitten_flip_literal",
    "kitten_flip_phases",
    "kitten_init",
    "kitten_no_ticks_limit",
    "kitten_randomize_phases",
    "kitten_release",
    "kitten_set_ticks_limit",
    "kitten_shrink_to_clausal_core",
    "kitten_shuffle_clauses",
    "kitten_solve",
    "kitten_status",
    "kitten_track_antecedents",
    "kitten_traverse_core_clauses",
    "kitten_traverse_core_ids",
    "kitten_unit",
    "kitten_value",
    "new_learned_klause",
];

fn rename_kitten_symbols(archive: &Path) -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let symbol_map = Path::new(&out_dir).join("kissat-kitten-symbols.map");
    let map = KITTEN_SYMBOLS
        .iter()
        .map(|symbol| format!("{symbol} kissat_{symbol}\n"))
        .collect::<String>();
    fs::write(&symbol_map, map)?;

    let objcopy = env::var("OBJCOPY").unwrap_or_else(|_| "objcopy".to_owned());
    let status = Command::new(objcopy)
        .arg(format!("--redefine-syms={}", symbol_map.display()))
        .arg(archive)
        .status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "objcopy failed with status: {}",
            status
        )));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    giputils::build::git_submodule_update()?;
    println!("cargo:rerun-if-changed=./kissat");
    let cb_path = copy_build("kissat", |src| {
        let status = Command::new("sh")
            .env("CC", "clang")
            .arg("configure")
            .arg("-fPIC")
            .arg("--competition")
            .current_dir(src)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "configure failed with status: {}",
                status
            )));
        }
        let num_jobs = env::var("NUM_JOBS").unwrap();
        let status = Command::new("make")
            .arg(format!("-j{num_jobs}"))
            .current_dir(src)
            .status()?;
        if !status.success() {
            return Err(io::Error::other(format!(
                "make failed with status: {}",
                status
            )));
        }
        rename_kitten_symbols(&src.join("build").join("libkissat.a"))?;
        Ok(())
    })?;
    println!(
        "cargo:rustc-link-search=native={}",
        cb_path.join("build").display()
    );
    println!("cargo:rustc-link-lib=static=kissat");
    Ok(())
}

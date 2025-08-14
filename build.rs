#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
use std::env;
use std::path::PathBuf;

extern crate bindgen;
extern crate cmake;

fn main() {
    setup_linking();
    build_teakra();
    generate_bindings();
}

fn setup_linking() {
    // Tell cargo to look for shared libraries in the specified directory
    let outPath = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}/build/src", outPath);

    // Tell cargo to tell rustc to link teakra and stdc++
    // Note: On Apple we have to link libc++, otherwise we have to link libstdc++
    let targetOS = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let cpp = if targetOS == "macos" { "c++" } else { "stdc++" };
    println!("cargo:rustc-flags=-lteakra_c -lteakra -l{}", cpp);
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./teakra/include/teakra")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_teakra() {
    let mut cfg = cmake::Config::new("./teakra");

    cfg.profile("Release");
    cfg.define("CMAKE_CONFIGURATION_TYPES", "Release");
    cfg.build_target("all");
    cfg.build();
}

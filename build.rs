extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/m1cycles.cpp");
    println!("cargo:rerun-if-changed=src/m1cycles.hpp");
    cc::Build::new()
        .cpp(true)
        .file("src/m1cycles.cpp")
        .flag("-std=c++17")
        .flag("-O2")
        .flag("-fno-tree-vectorize")
        .compile("m1cycles");

    println!("cargo:rerun-if-changed=wrapper.hpp");
    println!("cargo:rustc-link-lib=m1cycles");

    let bindings = bindgen::Builder::default()
        .clang_arg(r"-std=c++17")
        .header("wrapper.hpp")
        .dynamic_link_require_all(true)
        .allowlist_function("get_counters_checked")
        .allowlist_function("setup_performance_counters")
        .allowlist_type("performance_counters")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

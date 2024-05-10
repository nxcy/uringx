use std::env;
use std::path::PathBuf;

fn main() {
    pkg_config::probe_library("liburing-ffi").unwrap();
    bindgen::builder()
        .header("ffi.c")
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap()
}

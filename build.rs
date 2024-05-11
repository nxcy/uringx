use std::env;
use std::path::PathBuf;

use bindgen::{EnumVariation, NonCopyUnionStyle};

fn main() {
    pkg_config::probe_library("liburing-ffi").unwrap();
    bindgen::builder()
        .header("ffi.c")
        .allowlist_file("/usr/include/liburing.h")
        .allowlist_file("/usr/include/liburing/io_uring.h")
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .default_enum_style(EnumVariation::ModuleConsts)
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap()
}

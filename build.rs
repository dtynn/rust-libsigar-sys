extern crate cmake;

use std::path;

fn main() {
    let dst = cmake::Config::new("libsigar")
        .static_crt(true)
        .very_verbose(true)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        path::Path::new(&dst).join("lib").display(),
    );
    println!("cargo:rustc-link-lib=static=sigar");
}

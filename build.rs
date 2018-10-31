extern crate cmake;

use std::path;

#[cfg(target_env = "msvc")]
const STATIC_CRT: bool = true;

#[cfg(not(target_env = "msvc"))]
const STATIC_CRT: bool = false;

fn main() {
    let dst = cmake::Config::new("libsigar")
        .very_verbose(true)
        .static_crt(STATIC_CRT)
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        path::Path::new(&dst).join("lib").display(),
    );
    println!("cargo:rustc-link-lib=static=sigar");
}

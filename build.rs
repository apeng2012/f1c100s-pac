use std::env;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}

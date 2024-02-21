use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!(
        "cargo:rustc-link-search=native={}/lib/",
        env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    // Link to the correct target
    //
    if cfg!(feature = "kernel") {
        println!("cargo:rustc-link-lib=dylib=VMProtectDDK64");
    } else if cfg!(feature = "user") {
        println!("cargo:rustc-link-lib=dylib=VMProtectSDK64");
    } else {
        panic!("Unsupported architecture")
    }
}
use std::{env, fs, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Copy our memory.x into OUT_DIR so the linker finds it before any
    // crate-provided version (nrf52840-hal ships one starting at 0x0 which
    // would overwrite Adafruit's MBR + SoftDevice region).
    fs::copy("memory.x", out.join("memory.x")).unwrap();

    // Put OUT_DIR first in the linker search path.
    println!("cargo:rustc-link-search={}", out.display());

    // Re-run this script if either file changes.
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // Scope the memory map to THIS crate's own examples only. Using
    // `rustc-link-arg-examples` (instead of `rustc-link-search`) means the
    // search path is NOT propagated to downstream crates that depend on f411:
    // applications must supply their own memory.x. This keeps f411 a
    // pure board-support library while still letting its examples link.
    println!("cargo:rustc-link-arg-examples=-L{}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");
}

extern crate wayland_scanner;

use std::env::var;
use std::path::Path;

use wayland_scanner::{generate_code, Side};

// Example custom build script.
fn main() {
    // Location of the xml file, relative to the `Cargo.toml`
    let protocol_file = "./protocols/virtual-keyboard-unstable-v1.xml";

    // Target directory for the generate files
    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    generate_code(
        protocol_file,
        out_dir.join("virtual_keyboard_api.rs"),
        Side::Client, // Replace by `Side::Server` for server-side code
    );
    println!("cargo:rerun-if-changed=./protocols/");
    println!("cargo:rerun-if-changed=build.rs");
    //println!("cargo:rustc-link-lib=dylib=wayland-client"); // Is this necessary??
}

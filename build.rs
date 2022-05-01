use std::env::var;
use std::path::Path;

// Build script to generate the code from the xml file defining the protocol
fn main() {
    // Location of the xml file, relative to the `Cargo.toml`
    let protocol_file = "./protocols/virtual-keyboard-unstable-v1.xml";

    // Target directory for the generate files
    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    wayland_scanner::generate_code(
        protocol_file,
        out_dir.join("virtual_keyboard_api.rs"),
        wayland_scanner::Side::Client, // Replace by `Side::Server` for server-side code
    );
    println!("cargo:rerun-if-changed=./protocols/");
    println!("cargo:rerun-if-changed=build.rs");
}

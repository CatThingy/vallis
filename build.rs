use wayland_scanner::{generate_code, Side};

use std::path::Path;

fn main() {
    // Location of the xml file, relative to the `Cargo.toml`
    let protocol_path = Path::new("./protocol/river-layout-v3.xml");

    // Target directory for the generate files
    let out_file = Path::new("./generated/river_layout_v3.rs");
    generate_code(protocol_path, out_file, Side::Client);
}

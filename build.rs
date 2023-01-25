// The custom build script, needed as we use flatbuffers.

use flatc_rust;

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/world.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/world.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    }).expect("flatc");
}
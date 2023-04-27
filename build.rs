// The custom build script, needed as we use flatbuffers.

use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/server/data.proto3");
    prost_build::compile_protos(&["src/server/data.proto3"], &["src/"]).unwrap();
    println!("cargo:rerun-if-changed=src/world.fbs");
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("src/world.fbs")],
        out_dir: Path::new("target/flatbuffers/"),
        ..Default::default()
    })
    .expect("flatc");
}

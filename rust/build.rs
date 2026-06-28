extern crate prost_build;
extern crate protobuf_src;

use prost_build::Config;
use std::path::PathBuf;

const PROTO_FILES: &[&str] = &[
    "matsim/simulation/io/types/general.proto",
    "matsim/simulation/io/types/events.proto",
    "matsim/simulation/io/types/network.proto",
    "matsim/simulation/io/types/population.proto",
    "matsim/simulation/io/types/vehicles.proto",
];

fn main() {
    let proto_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("proto");
    let proto_files = PROTO_FILES
        .iter()
        .map(|proto| proto_root.join(proto))
        .collect::<Vec<_>>();

    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    println!("cargo:rerun-if-changed={}", proto_root.display());

    let mut config = Config::new();
    config.protoc_executable(protobuf_src::protoc());

    config.compile_protos(&proto_files, &[proto_root]).unwrap();
}

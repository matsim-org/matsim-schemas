extern crate prost_build;
extern crate protoc_bin_vendored;

use prost_build::Config;
use std::path::PathBuf;

const PROTO_FILES: &[&str] = &[
    "matsim/io/types/general.proto",
    "matsim/io/types/events.proto",
    "matsim/io/types/facilities.proto",
    "matsim/io/types/network.proto",
    "matsim/io/types/population.proto",
    "matsim/io/types/transit.proto",
    "matsim/io/types/vehicles.proto",
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
    // A prebuilt protoc, so that neither a system protoc nor a C++ build of protobuf is needed.
    config.protoc_executable(
        protoc_bin_vendored::protoc_bin_path()
            .expect("No bundled protoc compiler is available for this build platform"),
    );

    config.compile_protos(&proto_files, &[proto_root]).unwrap();
}

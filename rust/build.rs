extern crate prost_build;
extern crate protoc_bin_vendored;

use prost_build::Config;
use std::path::PathBuf;

/// Shared message types, always generated.
const TYPE_FILES: &[&str] = &[
    "matsim/io/types/general.proto",
    "matsim/io/types/events.proto",
    "matsim/io/types/facilities.proto",
    "matsim/io/types/network.proto",
    "matsim/io/types/population.proto",
    "matsim/io/types/transit.proto",
    "matsim/io/types/vehicles.proto",
];

/// Routing service API: messages, gRPC client and server, only with the feature `routing`.
#[cfg(feature = "routing")]
const ROUTING_FILES: &[&str] = &["matsim/services/routing/v1/routing.proto"];

fn main() {
    let proto_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("proto");

    #[allow(unused_mut)]
    let mut files = TYPE_FILES.to_vec();
    #[cfg(feature = "routing")]
    files.extend_from_slice(ROUTING_FILES);

    let proto_files = files
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
    // for the well-known types the services use (google/protobuf/empty.proto)
    let protoc_include = protoc_bin_vendored::include_path()
        .expect("The bundled protoc compiler's standard .proto include directory is unavailable");
    let include_dirs = [proto_root, protoc_include];

    // All files in one call, so that the services refer to the shared types generated here.
    #[cfg(feature = "routing")]
    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .compile_with_config(config, &proto_files, &include_dirs)
        .unwrap();

    #[cfg(not(feature = "routing"))]
    config.compile_protos(&proto_files, &include_dirs).unwrap();
}

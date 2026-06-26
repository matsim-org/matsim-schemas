extern crate prost_build;
extern crate protobuf_src;

use prost_build::Config;

fn main() {
    let proto_files = [
        "../proto/matsim/simulation/io/types/general.proto",
        "../proto/matsim/simulation/io/types/events.proto",
        "../proto/matsim/simulation/io/types/network.proto",
        "../proto/matsim/simulation/io/types/population.proto",
        "../proto/matsim/simulation/io/types/vehicles.proto",
    ];

    for proto in &proto_files {
        println!("cargo:rerun-if-changed={}", proto);
    }

    println!("cargo:rerun-if-changed=../proto");

    let mut config = Config::new();
    config.protoc_executable(protobuf_src::protoc());

    config.compile_protos(&proto_files, &["../proto"]).unwrap();
}

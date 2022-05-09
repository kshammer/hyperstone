use std::fs;
extern crate prost_build;

use prost_wkt_build::*;
use std::{env, path::PathBuf};

fn main() {
    let paths = fs::read_dir("src/proto")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|e| e.is_file())
        .collect::<Vec<_>>();

    // println!("{:?}", paths);
    prost_build::compile_protos(&paths, &["src/proto"]).unwrap();

    // let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    // let descriptor_file = out.join("descriptors.bin");
    // let mut prost_build = prost_build::Config::new();
    // prost_build
    //     .type_attribute(".", "#[derive(Serialize,Deserialize)]")
    //     .type_attribute(".", "#[typetag::serde(tag = \"@type\")]")
    //     .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
    //     .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
    //     .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
    //     .file_descriptor_set_path(&descriptor_file)
    //     .compile_protos(&paths, &["src/proto"])
    //     .unwrap();

    // let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

    // let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

    // prost_wkt_build::add_serde(out, descriptor);
}

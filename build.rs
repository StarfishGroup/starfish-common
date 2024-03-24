fn main() {
    #[cfg(feature = "rpc")]
    {
        use prost_wkt_build::*;
        use std::{env, path::PathBuf};

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let descriptor_file = out_dir.join("descriptors.bin");

        tonic_build::configure()
            .file_descriptor_set_path(&descriptor_file)
            .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
            .extern_path(".google.protobuf.Any", "::prost_wkt_types::Any")
            .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
            .extern_path(".google.protobuf.Value", "::prost_wkt_types::Value")
            .compile(&["proto/starfish.proto"], &["proto"])
            .unwrap();

        let descriptor_bytes = std::fs::read(descriptor_file).unwrap();

        let descriptor = FileDescriptorSet::decode(&descriptor_bytes[..]).unwrap();

        prost_wkt_build::add_serde(out_dir, descriptor);
    }
}

// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#![expect(missing_docs)]

use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(mesh::MeshPayload)]")
        .type_attribute(".", "#[mesh(prost)]")
        .service_generator(Box::new(mesh_build::MeshServiceGenerator::new()))
        .compile_protos(&["src/vmservice.proto"], &["src"])?;

    // There's an edge case where cargo build wont run if a new proto file is added to `src/`
    // but neither any existing proto files nor `build.rs` are updated to reference it.
    // That's acceptable, since it's a fairly pathological case.
    rerun_on_proto_files("src".as_ref())?;
    Ok(())
}

fn rerun_on_proto_files(dir: &Path) -> io::Result<()> {
    use std::ffi::OsStr;
    use std::fs::read_dir;

    // Could also use `iter.Filter` & `iter.Flatten` on `fs::ReadDir` to enumerate all `*.proto` files recursively.
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                rerun_on_proto_files(&path)?;
            } else if path.extension() == Some(OsStr::new("proto")) {
                println!("cargo::rerun-if-changed={}", path.display());
            }
        }
    }
    Ok(())
}

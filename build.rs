use std::io::Result;
use walkdir::WalkDir;
fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[cfg_attr(feature = \"flutter\", flutter_rust_bridge::frb(opaque))]",
    );
    let mut protos = Vec::new();
    for entry in WalkDir::new("src/proto/scatterbrain") {
        let entry = entry?;
        if entry.file_type().is_file()
            && entry.path().extension().unwrap().to_string_lossy() == "proto"
        {
            protos.push(entry.path().to_str().unwrap().to_owned());
        }
    }

    config.compile_protos(&protos, &["src/proto"])?;
    Ok(())
}

use std::io::Result;
fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[flutter_rust_bridge::frb(opaque)]",
    );
    config.compile_protos(&["src/scatterbrain.proto"], &["src/"])?;
    Ok(())
}

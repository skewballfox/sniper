use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .type_attribute(
            ".sniper.SnippetInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(&["../../proto/sniper.proto"], &["../../proto"])?;
    Ok(())
}

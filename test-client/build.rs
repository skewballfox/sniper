use std::io::Result;
use tonic_build;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .type_attribute(
            ".sniper.SnippetInfo",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile(&["../proto/sniper.proto"], &["../proto"])?;
    //println!("yeet");
    Ok(())
}

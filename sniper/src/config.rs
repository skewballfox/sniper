use directories::BaseDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use std::collections::HashMap;
use std::vec::Vec;

///This crate serves two purposes:
/// 1. store configuration settings and paths
/// 2. handle path based functionality
#[derive(Debug)]
pub(crate) struct SniperConfig {
    config_path: PathBuf,
    pub(crate) languages: HashMap<String, LanguageConfig>,
}
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct LanguageConfig {
    pub(crate) base_snippets: Vec<String>,
    #[serde(default = "not_initialized")]
    pub(crate) initialized: bool,
}

fn not_initialized() -> bool {
    false
}

#[derive(Deserialize, Clone, Debug)]
struct Loader {
    #[serde(rename = "settings", flatten)]
    pub(crate) language_settings: HashMap<String, LanguageConfig>,
}

impl SniperConfig {
    pub fn new() -> Self {
        let path = BaseDirs::new()
            .unwrap()
            .config_dir()
            .join(PathBuf::from(&"sniper"));
        let toml_file = &path.join("config.toml");
        println!("{:?}", toml_file);
        println!("config file loaded: {:?}", toml_file);
        let toml_data = fs::read_to_string(&toml_file).expect("failed to load file");
        let mut temp: Loader = toml::from_str(&toml_data).unwrap();

        Self {
            config_path: PathBuf::from(path),
            languages: temp.language_settings,
        }
    }

    pub fn get_snippet_data(&self, language: &str, snippet_set: &str) -> String {
        //TODO: actually handle errors in this function
        //its likely to actually generate them
        let snip_path = self.config_path.to_str().unwrap().to_owned()
            + "/"
            + language
            + &"/"
            + snippet_set
            + &".json";
        println!("{:?}", snip_path);
        fs::read_to_string(&snip_path).unwrap()
    }
}

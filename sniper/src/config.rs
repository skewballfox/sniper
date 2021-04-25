

use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

use std::collections::{HashMap,HashSet};
use std::vec::Vec;

///This crate serves two purposes:
/// 1. store configuration settings and paths
/// 2. handle path based functionality
#[derive(Debug)]
pub(crate) struct SniperConfig {
    config_path: PathBuf,   
    pub(crate) languages: HashMap<String,LanguageConfig>,

}
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct LanguageConfig {
    pub(crate) base_snippets: Vec<String>,
    #[serde(default="HashSet::new")]
    pub(crate) loaded_snippets: HashSet<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct Loader {
    #[serde(rename="settings",flatten)]
    language_settings: HashMap<String, LanguageConfig>,
}


impl SniperConfig {
    pub fn new() -> Self {
        let path= BaseDirs::new().unwrap().config_dir().join(PathBuf::from(&"sniper"));
        let toml_file = &path.join("config.toml");
        println!("{:?}",toml_file);
        
        println!("config file loaded: {:?}", toml_file);
        let toml_data = fs::read_to_string(&toml_file).expect("failed to load file");
        let temp: Loader=toml::from_str(&toml_data).unwrap();
        
        Self {
            config_path: PathBuf::from(path), 
            languages: temp.language_settings,
            
        }
        
    }
    fn load_config(&mut self){
        let toml_file = self.config_path.join("config.toml");
        println!("{:?}",toml_file);
        if toml_file.is_file(){
            println!("config file loaded: {:?}", toml_file);
            let toml_data = fs::read_to_string(&toml_file).expect("failed to load file");
            let temp: Loader=toml::from_str(&toml_data).unwrap();
            self.languages=temp.language_settings;
        }else {
                println!("check the path: {:?}", toml_file);
        }
    }
    
    
    pub fn get_snippet_data(&self, language: &str, snippet_set: &str)->String{
            
        //TODO: actually handle errors in this function
        //its likely to actually generate them    
        let snip_path=self.config_path.to_str().unwrap().to_owned()+"/"+language+&"/"+snippet_set+&".toml";
        println!("{:?}",snip_path);
        fs::read_to_string(&snip_path).unwrap()
    
        
    }

    pub fn added_snippets(&mut self, language:&str, snippet_set: String){
        self.languages[language].loaded_snippets.insert(snippet_set);
    }

}


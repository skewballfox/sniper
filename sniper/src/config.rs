

use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub struct SniperConfig {
    config_path: PathBuf,
    
    languages: HashMap<String,LanguageConfig>,

}
#[derive(Deserialize, Clone, Debug)]
struct LanguageConfig {
    base_snippets: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
struct Loader {
    #[serde(rename="settings",flatten)]
    language_settings: HashMap<String, LanguageConfig>,
}




pub fn ConfigLoader()-> SniperConfig {
    let path= BaseDirs::new().unwrap().config_dir().join(PathBuf::from(&"sniper"));
    let mut config = SniperConfig::new(&path.into_os_string().into_string().unwrap());
    config.load_config();
    config
}

impl SniperConfig {
    fn new(path: &str) -> Self {
        Self {
            config_path: PathBuf::from(path), 
            languages: HashMap::new(),
            
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

    pub fn get_base_snippets_path(self,language: &str)->Vec<String>{
        //if let Some(snippet_sets)= self.languages[language].base_snippets{
            if self.config.languages.contains_key(&language){
                let snippet_sets=self.languages[language].base_snippets;
                for (i, snip_set) in snippet_sets.iter().enumerate(){
                    snippet_sets[i]=self.config_path.join(PathBuf::from("/snippets/".to_owned()+language+snip_set)).into_os_string().into_string().unwrap();
                }
                snippet_sets
            }//TODO: figure out how to return an option
        //}
    }

}


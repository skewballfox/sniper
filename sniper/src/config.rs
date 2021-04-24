

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
    
    let mut config = SniperConfig::new();
    config.load_config();
    config
}

impl SniperConfig {
    pub fn new() -> SniperConfig {
        SniperConfig {
            config_path: BaseDirs::new().unwrap().config_dir().join(PathBuf::from(&"sniper")), 
            languages: HashMap::new(),
            
        }
        
    }
    pub fn load_config<'a>(&'a mut self) -> &'a mut SniperConfig {
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
        self
    }

    pub fn get_base_snippets_path(self,language: &str)->Option<Vec<String>>{
        //if let Some(snippet_sets)= self.languages[language].base_snippets{
            if self.languages.contains_key(language){
                let snip_sets=self.languages[language].base_snippets;
                let mut snip_data: Vec<String> =Vec::with_capacity(snip_sets.len());
                
                for (i, snip_set) in snip_sets.iter().enumerate(){
                    let snip_path=self.config_path.to_str().unwrap().to_owned()+&"/snippets/"+language+snip_set+&".toml";
                    snip_data.push(fs::read_to_string(&snip_path).unwrap());
                    
                }
                Some(snip_data)
            }else{
                None
            }
        //}
    }

}


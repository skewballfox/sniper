use crate::config::SniperConfig;
use crate::target::Target;
use crate::snippet::{Snippet,SnippetSets,Loader};

use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::HashSet;


#[derive(Debug)]
pub struct Sniper {
 
    config: SniperConfig,
    targets: HashSet<Target>,
    snippets: DashMap<String,Snippet>,
    snippet_sets: HashSet<SnippetSets>,
}

impl Sniper {
    pub fn new() -> Self {
        Self {
            config: SniperConfig::new(),
            targets: HashSet::new(),
            snippets: DashMap::new(), 
            snippet_sets: HashSet::new(),
        }
    }
    
    /// get, parse, rebuild, and return a snippet
    fn snipe(self, snippet: &str) {
        
        println!("todo");
        //return snippets[snippet].body;
    }
    /// load snippets from file
    fn load_snippets(&mut self, snippet_data: &str){
    
        
        let temp: Loader=toml::from_str(&snippet_data).unwrap();
        for pair in temp.snippets.iter(){
            self.snippets.insert(pair.0.to_owned(),pair.1.to_owned());
        }
        
    }
    /// drop a snippet set
    fn drop_snippets(&mut self) {
        
        println!("todo");
    }

    /// add a session to the list of currently tracked sessions
    pub fn add_target(&mut self, session_id: &str,uri: &str, language: &str){

        if self.config.languages.contains_key(language) {
            if self.config.languages[language].loaded_snippets.is_empty() {
                for snippet_set in self.config.languages[language].base_snippets.iter(){
         
                    //NOTE: in future need to handle error, where base snippets
                    //defined in config isn't found (in appropriate directory)
                    let snippet_data= self.config.get_snippet_data(language,snippet_set);
                    self.load_snippets(&snippet_data);
                    self.config.added_snippets(language,snippet_set.to_string());  
                }
            }
        }
        let target=Target::new(&session_id,&uri,&language);
        self.targets.insert(target);
    }

    //fn update_target(&mut self)
       
        
    /// drop a target exit sniper if no targets left
    /// drop a snippet set if no longer required by any targets
    fn drop_target(&mut self, target: Target){
        
        println!("todo");
        //self.targets.remove(&Target);
        // if self.targets.empty(){
        //     std::process::exit();
        // }
    }

}
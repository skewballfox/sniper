use crate::config::SniperConfig;
use crate::target::TargetData;
use crate::snippet::{Snippet,SnippetSet,Loader};

use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::{HashSet,HashMap};


#[derive(Debug)]
pub struct Sniper {
 
    pub(crate) config: SniperConfig,
    targets: HashMap<(String,String),TargetData>,
    pub(crate) snippets: DashMap<(String,String),Snippet>,
    snippet_sets: HashMap<(String,String),SnippetSet>,
}

impl Sniper {
    pub fn new() -> Self {
        Self {
            config: SniperConfig::new(),
            targets: HashMap::new(),
            snippets: DashMap::new(), 
            snippet_sets: HashMap::new(),
        }
    }
    
    /// get, parse, rebuild, and return a snippet
    fn snipe(self, snippet: &str) {
        
        println!("todo");
        //return snippets[snippet].body;
    }
    
    //pub fn add_snippets()
    
    /// load snippets from file
    fn load_snippets(&mut self,language: &str,snip_set_name: &str, snippet_data: &str){
    
        
        let temp: Loader=toml::from_str(&snippet_data).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.to_string(),snippet_key.to_string()),snippet.to_owned());
            snippet_set.push(snippet_key.to_string());
        }
        self.snippet_sets.insert((language.to_string(),snip_set_name.to_string()),SnippetSet::new(snippet_set));
        
    }
    
    /// drop snippets tied to a given snippet set
    fn drop_snippets(&mut self, language: &str, snip_set_to_drop: String) {
        for snippet_key in self.snippet_sets[&(language.to_string(),snip_set_to_drop)].contents.iter(){
            self.snippets.remove(&(language.to_string(),snippet_key.to_string()));
        }
    }

    /// add a session to the list of currently tracked sessions
    pub fn add_target(&mut self, session_id: &str,uri: &str, language: &str){

        if self.config.languages.contains_key(language) {
            let mut target_data=TargetData::new(&language);
            
            if !self.config.languages[language].initialized {
                for snippet_set in self.config.languages[language].base_snippets.clone().iter(){
                    //NOTE: in future need to handle error, where base snippets
                    //defined in config isn't found (in appropriate directory)
                    let snippet_data= self.config.get_snippet_data(language,snippet_set);
                    self.load_snippets(language,snippet_set,&snippet_data);
                    target_data.loaded_snippets.insert(snippet_set.to_string());

                }
                self.config.language_initialized(language);
            } else {//the base sets for this language have already been loaded
                for snippet_set in self.config.languages[language].base_snippets.clone().iter(){
                    self.snippet_sets.get_mut(&(language.to_string(),snippet_set.to_string())).unwrap().increment_target();
                }
            }
            self.targets.insert((session_id.to_string(),uri.to_string()),target_data);
            //should only track a target if it is in a supported language
            //should have some way of mitigating request for adding nonviable targets
            //client side
            
        }
    }
    
    //fn update_target(&mut self, )
    
    //fn add_snippet_set(&mut self)
       
        
    /// drop a target exit sniper if no targets left
    /// drop a snippet set if no longer required by any targets
    fn drop_target(&mut self, session_id: &str,uri: &str, language: &str){
        
        if self.targets.contains_key(&(session_id.to_string(),uri.to_string())){
            //consider using drain filter in the future:
            //https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.drain_filter
            if let Some(target_data)=self.targets.remove(&(session_id.to_string(),uri.to_string())){    
                for snip_set in target_data.loaded_snippets.iter(){
                
                    let drop_snippets_flag=self.snippet_sets.get_mut(&(language.to_string(),snip_set.to_string())).unwrap().decrement_target();
                        if drop_snippets_flag {
                            self.drop_snippets(language,snip_set.to_string())
                        }
                }
            }
            
        }
    }

}
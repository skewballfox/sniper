use crate::config::{ConfigLoader,SniperConfig};
use crate::target::Target;
use crate::snippet::{Snippet,SnippetSets,Loader};

use dashmap::DashMap;
use rayon::prelude::*;
use std::collections::{HashMap,HashSet};
use std::vec::Vec;

#[derive(Debug)]
pub struct Sniper<'a> {
    //TODO: since config is now static, switch to passing sessions
    config: &'a SniperConfig,
    //
    targets: HashSet<Target>,
    snippets: DashMap<String,Snippet>,
    snippet_sets: HashSet<SnippetSets>,
}

impl Sniper<'_> {
    pub fn new() -> Self {
        Self {
            config: SniperConfig::new().load_config(),
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
        println!("{:?}",snippet_data);
        
        println!("snippet data loaded: {:?}", snippet_data);
        
        let temp: Loader=toml::from_str(&snippet_data).unwrap();
        for pair in temp.snippets.iter(){
            self.snippets.insert(pair.0,pair.1);
        }
        
        println!("todo");
    }
    /// drop a snippet set
    fn drop_snippets(&mut self) {
        
        println!("todo");
    }

    /// add a session to the list of currently tracked sessions
    pub fn add_target(&mut self, session_id: &str,uri: &str, language: &str){

        //NOTE: Leaving off here, and in get base snippet path in config.rs
        //this solution is bad and you should feel bad
        if self.snippet_sets.is_empty(){//TODO: figure out what to do about partial matches
            if let Some(snippet_files)=self.config.get_base_snippets_path(&language){
                for snippet_file in snippet_files.iter(){
                    self.load_snippets(&snippet_file);
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
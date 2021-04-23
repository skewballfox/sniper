use crate::config::SniperConfig;
use crate::target::Target;
use crate::snippet::{Snippet,SnippetSets,Loader};

use dashmap::DashMap;
use std::collections::{HashMap,HashSet};
use std::vec::Vec;

#[derive(Debug)]
pub struct Sniper {
    //TODO: since config is now static, switch to passing sessions
    config: SniperConfig,
    //
    targets: HashSet<Target>,
    snippets: DashMap<String,Snippet>,
    snippet_sets: HashSet<SnippetSets>,
}

impl Sniper {
    pub fn new(config: SniperConfig) -> Self {
        Self {
            config: config,
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
    fn load_snippets(&mut self, language: &str){
        
        println!("todo");
    }
    /// drop a snippet set
    fn drop_snippets(&mut self) {
        
        println!("todo");
    }

    /// add a session to the list of currently tracked sessions
    fn add_target(&mut self, target: Target){
        //NOTE: Leaving off here, and in get base snippet path in config.rs
        let language=target.get_language();
        self.targets.insert(target);
        
        let snippet_files=self.config.get_base_snippets_path(&language)
        println!("{:?}",snippet_files);
    }
       
        
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
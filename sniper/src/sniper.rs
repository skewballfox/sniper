use service::SniperService;

use crate::config::SniperConfig;
use crate::target::TargetData;
use crate::rifle::Rifle;




use std::collections::{HashMap};


#[derive(Debug)]
pub struct Sniper {
    pub(crate) config: SniperConfig,
    pub(crate) targets: HashMap<(String,String),TargetData>,
    pub(crate) rifle: Rifle,
}

impl Sniper {
    pub fn new() -> Self {
        Self {
            config: SniperConfig::new(),
            targets: HashMap::new(),
            rifle: Rifle::new()
        }
    }
    
    /// get, parse, rebuild, and return a snippet
    pub fn snipe(&mut self, language: &str, snippet_key: &str) -> Option<Vec<String>> {
        if let Some(round)=self.rifle.fire(language,snippet_key){
            Some(round)
        } else {
            None
        }
        
        //return snippets[snippet].body;
    }
    /*fn rebuild_snippet(&mut self, language: &str, snippet: &mut Snippet) -> Snippet {
        unimplemented!();
    }*/
    
    
    /// add a session to the list of currently tracked sessions
    pub fn add_target(&mut self, session_id: &str,uri: &str, language: &str){

        if self.config.languages.contains_key(language.into()) {
            let mut target_data=TargetData::new(language.into());
            
            if !self.config.languages[language].initialized {
                for snippet_set in self.config.languages[language.into()].base_snippets.clone().iter(){
                    //NOTE: in future need to handle error, where base snippets
                    //defined in config isn't found (in appropriate directory)
                    let snippet_data= self.config.get_snippet_data(language.into(),snippet_set);
                    self.rifle.load(language.into(),&snippet_set.to_string(),&snippet_data.to_string());
                    target_data.loaded_snippets.insert(snippet_set.to_string());
                }
                self.config.language_initialized(language.into());
            } else {//the base sets for this language have already been loaded
                for snippet_set in self.config.languages[language.into()].base_snippets.clone().iter(){
                    self.rifle.snippet_sets.get_mut(&(language.into(),snippet_set.into())).unwrap().increment_target_count();
                }
            }
            self.targets.insert((session_id.into(),uri.into()),target_data);
            //should only track a target if it is in a supported language
            //should have some way of mitigating request for adding nonviable targets
            //client side
            
        }
    }
    
    
       
        
    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    /// exit sniper if no targets left
    pub(crate) fn drop_target(&mut self, session_id: &str,uri: &str,language:&str){
        let target_key=&(session_id.to_string(),uri.to_string());
        
        if self.targets.contains_key(&(session_id.into(),uri.into())){
            //consider using drain filter in the future:
            //https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.drain_filter
            if let Some(target_data)=self.targets.remove(&(session_id.into(),uri.into())){    
                for snip_set in target_data.loaded_snippets.iter(){
        
                    let drop_snippets_flag=self.rifle.snippet_sets.get_mut(&(language.into(),snip_set.into())).unwrap().decrement_target_count();
                    if drop_snippets_flag {
                        self.rifle.unload(&language,snip_set)
                    }
                }
            }
            
        }
        if self.targets.is_empty(){
            println!("todo");
            //sys.exit(0);
        }
    }

    pub fn get_triggers(self,session_id: &str,uri: &str)-> Vec<String> {
        let language=self.targets.get(&(session_id.to_string(),uri.to_string())).unwrap().language.clone();
        let sets=self.targets.get(&(session_id.to_string(),uri.to_string()))
        .unwrap().loaded_snippets.clone().into_iter().collect();
        self.rifle.get_triggers(&language, sets)
    }

}
use std::sync::Arc;

use dashmap::DashMap;
use futures::lock::Mutex;

use sniper_common::service::SniperService;
use tarpc::{
    context,
    server::{self, Incoming, Channel},
};


use crate::{config::SniperConfig, snippet_manager::SnippetManager, target::TargetData};

#[derive(Clone)]
pub(crate) struct SniperServer {
    pub(crate) config: Arc<Mutex<SniperConfig>>,
    pub(crate) targets: Arc<DashMap<(String,String),TargetData>>,
    pub(crate) snip_lock: Arc<tokio::sync::RwLock<SnippetManager>>,
}


impl SniperServer {
    pub fn new( config:Arc<Mutex<SniperConfig>>,targets:Arc<DashMap<(String,String),TargetData>>,snip_lock: Arc<tokio::sync::RwLock<SnippetManager>>) -> Self { 
        Self { 
            config,
            targets,
            snip_lock
        } 
    }
}
#[tarpc::server]
impl SniperService for SniperServer{
    
    /// add a session to the list of currently tracked sessions
    async fn add_target(self,_:context::Context, session_id: String, uri: String, language: String) {
        println!("adding target: {:?},{:?},{:?}",session_id,uri,language);
        //let sniper=self.snip_lock.read().await;
        let mut config=self.config.lock().await;
        println!("loaded vars");
        //let targets=&*self.targets;
        if config.languages.contains_key(&language) {
            let mut target_data=TargetData::new(&language);
            
            if !config.languages[&language].initialized {
                println!("config contains language {:?}",language);
                let mut sniper=self.snip_lock.write().await;
                println!("got write lock for snippet manager");
                for snippet_set in config.languages[&language].base_snippets.clone().iter(){
                    //NOTE: in future need to handle error, where base snippets
                    //defined in config isn't found (in appropriate directory)
                    let snippet_data= config.get_snippet_data(&language,snippet_set);
                    println!("loading snippet data into sniper");
                    sniper.load(&language,&snippet_set.to_string(),&snippet_data.to_string());
                    target_data.loaded_snippets.insert(snippet_set.to_string());
                }
                config.language_initialized(&language);
            } else {//the base sets for this language have already been loaded
                for snippet_set in config.languages[&language].base_snippets.clone().iter(){
                    //copy the names of the already loaded snippet sets to target data
                    target_data.loaded_snippets.insert(snippet_set.to_string());
                }
            }
            &self.targets.insert((session_id.into(),uri.into()),target_data);
            //should only track a target if it is in a supported language
            //should have some way of mitigating request for adding nonviable targets
            //client side
        }
        println!("target_added")
    }

    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    /// exit sniper if no targets left
    async fn drop_target(self,_:context::Context,session_id: String, uri: String,language:String) {
        let target_key=&(session_id.to_string(),uri.to_string());
        let snippet_manager=self.snip_lock.read().await;
        println!("dropping target: {:?}",target_key);
        
        if self.targets.contains_key(target_key){
            //consider using drain filter in the future:
            //https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.drain_filter
            if let Some(target_data)=self.targets.remove(&(session_id,uri)){    
                for snip_set in target_data.1.loaded_snippets.iter(){
        
                    let drop_snippets_flag=snippet_manager.snippet_sets.get_mut(&(language.to_string(),snip_set.to_string())).unwrap().decrement_target_count();
                    if drop_snippets_flag {
                        let mut snippet_manager=self.snip_lock.write().await;
                        snippet_manager.unload(&language,&snip_set)
                    }
                }
            }
            
        }
        if self.targets.is_empty(){
            println!("todo");
            //sys.exit(0);
        }
    }

    /*async fn target_add_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }

    async fn target_drop_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }
    */


    async fn get_triggers(self,_:context::Context,session_id: String, uri: String)-> Vec<String> {
        let language=self.targets.get(&(session_id.to_string(),uri.to_string())).unwrap().language.clone();
        let snippet_manager=self.snip_lock.read().await;
        let sets: Vec<String>=self.targets.get(&(session_id.to_string(),uri.to_string()))
        .unwrap().loaded_snippets.clone().into_iter().collect();
        println!("triggers requested");
        let mut requested_triggers=Vec::new();
        println!("sets: {:?}",sets);
        sets.iter().into_iter().for_each(|set| {
            requested_triggers.append(&mut snippet_manager.snippet_sets.get(&(language.to_string(),set.into())).unwrap().contents.clone());
        });
        println!("requested triggers: {:?}",requested_triggers);
        let mut triggers:Vec<String>=Vec::with_capacity(requested_triggers.len());
        
        requested_triggers.into_iter().for_each(|snippet| {
            
            triggers.push(snippet_manager.snippets.get(&(language.clone(),snippet)).unwrap().prefix.clone());
            //let trigger=self.snippets.get(&(language.into(),snippet.into())).unwrap().prefix.clone();
            //return trigger.to_string()
        });
        println!("triggers: {:?}",triggers);
        return triggers
    }
    
    async fn get_snippet(self,_:context::Context,language: String, snippet_name: String) -> Option<Vec<String>> {
        let snippet_key=&(language.to_string(),snippet_name.to_string());
        let snippet_manager=self.snip_lock.read().await; 
        let mut assembly_required=false;
        let mut not_found=false;
        let mut snippet_body=Vec::new();
        println!("{:?} requested",snippet_name);
        if snippet_manager.snippets.contains_key(snippet_key) {
            if snippet_manager.snippets.get(snippet_key).unwrap().requires_assembly{
                assembly_required=true;
            } else {
                snippet_body=snippet_manager.snippets.get(snippet_key).unwrap().body.clone()
            }
        } else {
            not_found=true;
        }
        drop(snippet_manager);
        if not_found {
            None
        } else {
            
            if assembly_required {
                //only acquire writelock when necessary
                let mut snippet_manager=self.snip_lock.write().await;
                snippet_manager.rebuild_snippets(&language,snippet_name.into());
                snippet_body=snippet_manager.snippets.get(snippet_key).unwrap().body.clone();
            }
            Some(snippet_body)
        }
    }
}
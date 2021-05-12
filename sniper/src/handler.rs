use std::sync::Arc;

use dashmap::DashMap;
use futures::lock::Mutex;
use service::SniperService;
use tarpc::{
    context,
    server::{self, Incoming, Channel},
};


use crate::{config::SniperConfig, sniper::{self, Sniper}, target::TargetData};

#[derive(Clone)]
pub(crate) struct Spotter {
    pub(crate) config: Arc<Mutex<SniperConfig>>,
    pub(crate) targets: Arc<DashMap<(String,String),TargetData>>,
    //socket_address: PathBuf,
    pub(crate) sniper_lock: Arc<tokio::sync::RwLock<Sniper>>,
}


impl Spotter {
    pub fn new( config:Arc<Mutex<SniperConfig>>,targets:Arc<DashMap<(String,String),TargetData>>,sniper_lock: Arc<tokio::sync::RwLock<Sniper>>) -> Self { 
        Self { 
            config,
            targets,
            sniper_lock
        } 
    }
}
#[tarpc::server]
impl SniperService for Spotter{
    
    /// add a session to the list of currently tracked sessions
    async fn add_target(self,_:context::Context, session_id: String, uri: String, language: String) {
        println!("adding target");
        //let sniper=self.sniper_lock.read().await;
        let mut config=self.config.lock().await;
        println!("loaded vars");
        //let targets=&*self.targets;
        if config.languages.contains_key(&language) {
            let mut target_data=TargetData::new(&language);
            
            if !config.languages[&language].initialized {
                println!("config contains language {:?}",language);
                let mut sniper=self.sniper_lock.write().await;
                println!("got write lock for sniper");
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
                    let sniper=self.sniper_lock.read().await;
                    sniper.snippet_sets.get_mut(&(language.to_string(),snippet_set.into())).unwrap().increment_target_count();
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
        let sniper=self.sniper_lock.read().await;
       
        
        if self.targets.contains_key(target_key){
            //consider using drain filter in the future:
            //https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.drain_filter
            if let Some(target_data)=self.targets.remove(&(session_id,uri)){    
                for snip_set in target_data.1.loaded_snippets.iter(){
        
                    let drop_snippets_flag=sniper.snippet_sets.get_mut(&(language.to_string(),snip_set.to_string())).unwrap().decrement_target_count();
                    if drop_snippets_flag {
                        let mut sniper=self.sniper_lock.write().await;
                        sniper.unload(&language,&snip_set)
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
        let sniper=self.sniper_lock.read().await;
        let sets: Vec<String>=self.targets.get(&(session_id.to_string(),uri.to_string()))
        .unwrap().loaded_snippets.clone().into_iter().collect();
        
        let mut requested_triggers=Vec::new();
        
        sets.iter().into_iter().for_each(|set| {
            requested_triggers.append(&mut sniper.snippet_sets.get(&(language.to_string(),set.into())).unwrap().contents.clone());
        });
        
        let mut triggers:Vec<String>=Vec::with_capacity(requested_triggers.len());
        
        requested_triggers.into_iter().for_each(|snippet| {
            
            triggers.push(sniper.snippets.get(&(language.clone(),snippet)).unwrap().prefix.clone());
            //let trigger=self.snippets.get(&(language.into(),snippet.into())).unwrap().prefix.clone();
            //return trigger.to_string()
        });
        return triggers
    }
    
    async fn get_snippet(self,_:context::Context,language: String, snippet_name: String) -> Option<Vec<String>> {
        let snippet_key=&(language.to_string(),snippet_name.to_string());
        let mut sniper=self.sniper_lock.write().await; 
        //let mut assembly_required=false;
        println!("{:?} requested",snippet_name);
        if sniper.snippets.contains_key(snippet_key){
            if sniper.snippets.get(snippet_key).unwrap().requires_assembly{
                //let mut sniper=self.sniper_lock.write().await;
                sniper.rebuild_snippets(&language,snippet_name.into());
                
            }
    
            
            Some(sniper.snippets.get(snippet_key).unwrap().body.clone())
        } else {
            None
        }
    }
}
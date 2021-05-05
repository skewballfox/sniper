#[macro_use]
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelIterator,IndexedParallelIterator,ParallelIterator,IntoParallelRefIterator};
use priority_queue::PriorityQueue;
use crate::snippet::{Snippet,SnippetSet,Loader};
use crate::snippetParser::{SnipComponent,SnippetBuildMetadata};

use dashmap::DashMap;
use regex::Regex;

use std::collections::{HashMap};
use std::sync::{Arc,Mutex};
//good artist copy, great artist steal
//https://stackoverflow.com/questions/51344951/how-do-you-unwrap-a-result-on-ok-or-return-from-the-function-on-err
//TODO: consider using functions defined in above link in util.rs to have easy integration of error handling across sniper
//used to make 
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    }
}
/*macro_rules! get_sub_str {
    ( $slice:ident,$re:ident,$old_start:ident ) => {
        let temp_indices=$re.find($slice);
        let sub_str=((&$slice[temp_indices..temp_indices.end()]);
        $old_start=temp_indices.end()
    }
}*/

///This struct stores snippets, tracks the sets in play, and handles all logic related to snippet management
/// such as removal, and rebuilding the snippet
/// p.s. these word puns are probably getting out of hand at this point
#[derive(Debug)]
pub(crate) struct Rifle {
    pub(crate) snippets: DashMap<(String,String),Snippet>,
    pub(crate) snippet_sets: HashMap<(String,String),SnippetSet>,
}

impl Rifle {
    pub fn new() -> Self {
        Self {
            snippets: DashMap::new(),
            snippet_sets: HashMap::new(), 
        }
    }

    pub fn load(&mut self,language: &str,snip_set_name: &str, snippet_data: &str){
    
        let temp: Loader = serde_json::from_str(snippet_data.into()).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.to_string(),snippet_key.to_owned()),snippet.to_owned());
            snippet_set.push(snippet_key.to_owned());
        }
        self.snippet_sets.insert((language.into(),snip_set_name.into()),SnippetSet::new(snippet_set));
        let mut buildmap=DashMap::with_capacity(snippet_set.len());
        let q_guard=Arc::new(Mutex::new(PriorityQueue::with_capacity(snippet_set.len())));

        for snippet_name in snippet_set.par_iter(){
            let snipbuild=self.parse_snippet(language.into(),snippet_name);
            
            let mut snipq=q_guard.lock().unwrap();

            if let Some(P)= *snipq.get_priority(snippet_name){
                *snipq.change_priority(snippet_name,P+1);
            }else {
                *snipq.push(snippet_name,0);
            }
            
            for sub_name in snipbuild.sub_snippets.iter()
                if let Some(P)= *snipq.get_priority(sub_name){
                    *snipq.change_priority(sub_name,P+1);
                }else {
                    *snipq.push(sub_name,0)
                }
            buildmap.insert(snippet_name,snipbuild);
        }
        
        
    }
    pub fn unload(&mut self, language: &str, snip_set_to_drop: &str) {
        for snippet_key in self.snippet_sets[&(language.into(),snip_set_to_drop.into())].contents.iter(){
            self.snippets.remove(&(language.to_string(),snippet_key.to_string()));
        }
        self.snippet_sets.remove(&(language.into(),snip_set_to_drop.into()));
    }

    //TODO: probably need to change the output format
    pub fn fire(&mut self, language: &str,snippet_name: &str) -> Option<Vec<String>> { 
        if self.snippets.contains_key(&(language.to_string(),snippet_name.to_string())){
            
        } else {
            None
        }
    }
    
    async fn parse_snippet(&self, language: &str,snippet_name: &str){
        //NOTE: while having more than one mutable reference inside a dashmap can risk deadlocks when multithreading,
        //there is no risk associated with multiple immutable ones
        //therefor I'm splitting the snippet rebuild process into two parts: parse_snippet and build_snippet
        //parse_snippet will generate metadata for snippet builds(asynchronously) 
        lazy_static! {
            static ref digit: Regex = Regex::new(r"[[0-9]&&[^a-zA-Z]]+").unwrap();
            //TODO: deal with escaped characters such as \$ in bash
            static ref modification_needed: Regex = Regex::new(r"(\$\{?\d+)|@").unwrap();
            static ref snippet_finder: Regex = Regex::new("[[a-zA-Z0-9/]]+").unwrap();
            static ref snippet_args_finder: Regex = Regex::new(r"\(.*\)}").unwrap();
        }
        let mut build_data=Vec::with_capacity(self.snippets.get(&(language.into(),snippet_name.to_string())).unwrap().body.len());
        let mut sub_snippet_count=0;
        let mut snippet_stack=Vec::new();
        self.snippets.get(&(language.into(),snippet_name.to_string())).unwrap().body.into_par_iter().enumerate().for_each(|(line_index,line)| {
            let mut line_data=Vec::new();
            for sub_match in modification_needed.find_iter(&line.clone()){
                let lead_char=line[sub_match.start()..sub_match.end()].chars().nth(0).unwrap();

                match lead_char{
                    '$'=> {
                        let indices=digit.find(&line[sub_match.start()..sub_match.end()]).unwrap();
                        line_data.push(SnipComponent::tabstop{start:sub_match.start()+indices.start(),end:sub_match.start()+indices.end()});

                    }
                    '@'=>{
                        let indices=snippet_finder.find(&line[end..]).unwrap();
                        let sub_snippet_name=&line[end+snippet_indices.start()..end+snippet_indices.end()];
                        snippet_stack.push(sub_snippet_name);
                        sub_snippet_count+=1;
                        let sub_snippet_name=&line[sub_match.end()+indices.start()..sub_match.end()+indices.end()];
                        line_data.push(SnipComponent::sub_snippet{start:sub_match.end()+indices.start(),end:sub_match.end()+indices.end(),name=sub_snippet_name});
                    }
                    _=>{
                        panic!("Zoinks Scoob! That wasn't supposed to happen");
                    }

                }
            }
            build_data[line_index]=line_data;
        });

        new::SnippetBuildMetadata(snippet_name,sub_snippet_count)
    }

    fn build_snippet(&mut self, language: &str, build_data: Vec<SnippetBuildMetadata>) {

    }
    
    

}
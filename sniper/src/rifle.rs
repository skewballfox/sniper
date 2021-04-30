#[macro_use]
use lazy_static::lazy_static;

use crate::snippet::{Snippet,SnippetSet,Loader};


use dashmap::DashMap;
use regex::Regex;

use std::collections::{HashMap};
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

    pub fn load<S: Into<String>>(&mut self,language: S,snip_set_name: S, snippet_data: S){
    
        let temp: Loader = toml::from_str(&snippet_data.into()).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.into(),snippet_key.to_owned()),snippet.to_owned());
            snippet_set.push(snippet_key.to_owned());
        }
        self.snippet_sets.insert((language.into(),snip_set_name.into()),SnippetSet::new(snippet_set));
        
    }
    pub fn unload<S: Into<String>>(&mut self, language: S, snip_set_to_drop: S) {
        for snippet_key in self.snippet_sets[&(language.into(),snip_set_to_drop.into())].contents.iter(){
            self.snippets.remove(&(language.into(),snippet_key.to_string()));
        }
        self.snippet_sets.remove(&(language.into(),snip_set_to_drop.into()));
    }

    //TODO: probably need to change the output format
    pub fn fire<S: Into<String>>(&mut self, language: S,snippet_name: S) -> Option<Vec<String>> { 
        if self.snippets.contains_key(&(language.into(),snippet_name.into())){
            return Some(self.chamber_snippet(&language.into(),snippet_name,0,""));
        } else {
            return None
        }
    }

    
    fn chamber_snippet(&mut self, language: &str, snippet_name: &str,offset: i32,snippet_args: &str) -> Vec<String> {
        lazy_static! {
            static ref digit: Regex = Regex::new("/d+").unwrap();
            //TODO: deal with escaped characters such as \$ in bash
            static ref modification_needed: Regex = Regex::new(r#"\\$(\d+|\{\d+)|\\@"#).unwrap();
            static ref snippet_finder: Regex = Regex::new("[a-zA-Z0-9;]+").unwrap();
            static ref snippet_args_finder: Regex = Regex::new(r#"\\(.*\\)}"#).unwrap();
        }
        if !self.snippets.get(&(language.into(),snippet_name.to_string())).unwrap().requires_assembly {
            return self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().body.clone();
        }
        let mut assembled_snippet: Vec<String>=Vec::new();
        let mut sub_string=String::new();
        let mut start=0;
        let mut end=0;
        let mut bracket=0;
        let mut tabstop_count=offset.clone();
        let mut snippet_args="";
        &mut self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().body.iter().for_each(|line| {
 
            start=0;
            sub_string=String::with_capacity(line.len());
            //sub_res="";
            for submatch in modification_needed.find_iter(line){
                if submatch.start()>start {
                    sub_string.push_str(&line[start..submatch.start()]);
                }
                start=submatch.start();
                end=submatch.end();
                let mut sub_chars = line[start..end].chars();
                //I only care about two things server-side: tabstops and snippets
                match sub_chars.next(){
                    Some('$')=>{//if placeholder was found
                        if offset==0{
                            tabstop_count+=1;
                            sub_string.push_str(&line[start..end]);
                        } else {
                            let digit_indices=digit.find(&line[start+1..end]).unwrap();
                            let mut tabstop=(line[digit_indices.start()..digit_indices.end()]).parse::<i32>().unwrap();//turbofish
                            tabstop+=offset;
                            
                            //push everything upto tabstop, followed by new tabstop
                            sub_string.push_str(&line[start..digit_indices.start()]);
                            sub_string.push_str(&tabstop.to_string());
                            //sub_string.push_str(&line[digit_indices.end..end]);
                            //TODO: handle embedded tabstops
                            //now sort through the rest of the line
                            start=digit_indices.end();
                        }

                    }
                    Some('@')=>{//if snippet was found

                        let snippet_indices=snippet_finder.find(&line[start+1..]).unwrap();
                        let sub_snippet_name=&line[snippet_indices.start()..snippet_indices.end()];
                        start=snippet_indices.end();//ex: @if@elif@else
                        if self.snippets.contains_key(&(language.into(),sub_snippet_name.to_string())){
                            //NOTE: going off the assumption that a snippet should be in brackets
                            //only if it has tab_args
                            if sub_chars.next()==Some('{'){
                                let args_indices=snippet_args_finder.find(&line[snippet_indices.end()..]).unwrap();
                                if snippet_indices.end()==args_indices.start(){
                                    snippet_args=&line[args_indices.start()..args_indices.end()];
                                    start=args_indices.end()+1;
                                }
                            }
                            ;
                            assembled_snippet.append(&mut self.chamber_snippet(language.into(),sub_snippet_name,tabstop_count,snippet_args));
                        }
                    }
                    
                    std::option::Option::Some(_) => {}
                    std::option::Option::None => {}
                }//on to next $ or @ in line, if any
            }
            
            //nothing worth mentioning was found

            if sub_string.is_empty(){
                assembled_snippet.push(line.to_string())
            } else {
                sub_string.push_str(&line[end..]);
                assembled_snippet.push(sub_string);
            }
            start=0;
            
        
        });//no more lines
        *self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().body=assembled_snippet.clone();
        *self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().requires_assembly=false;
        return assembled_snippet
    }

}
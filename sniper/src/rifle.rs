#[macro_use]
use lazy_static;

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
        if self.snippets.contains_key(&(language.into(),snippet_name.into()){
            return Some(self.chamber_snippet(&language.into(),&mut *self.snippets.get_mut(&(language.into(),snippet_name.into())).unwrap(),0,""));
        } else {
            return None
        }
    }

    
    fn chamber_snippet(&mut self, language: &str, snippet: &mut Snippet,offset: i32,snippet_args: &str) -> Vec<String> {
        lazy_static! {
            static ref digit: Regex = Regex::new("/d+").unwrap();
            //TODO: deal with escaped characters such as \$ in bash
            static ref shit_i_care_about: Regex = Regex::new(r#"\\$(\d+|\{\d+)|\\@"#).unwrap();
            static ref snippet_finder: Regex = Regex::new("[a-zA-Z0-9;]+").unwrap();
            static ref snippet_args_finder: Regex = Regex::new(r#"\\(.*\\)}"#).unwrap();
        }
        if !snippet.requires_assembly {
            return snippet.body.clone();
        }
        let mut assembled_snippet: Vec<String>=Vec::new();
        let mut sub_string=String::new();
        let mut start=0;
        let mut bracket=0;
        let mut tabstop_count=offset.clone();
        for line in snippet.body.iter(){
 
            start=0;
            sub_string=String.with_capacity(line.len());
            sub_res=""
            for (sub_start,sub_end) in shit_i_care_about.find(line).iter(){
                if indices.start()>start {
                    sub_string.push_str(&line[start..sub_start]);
                }
                start=indices.start();
                let mut sub_chars=line[sub_start..sub_end].chars()
                //I only care about two things server-side: tabstops and snippets
                match (chars.next()){
                    "$"=>{//if placeholder was found
                        
                        let digit_indices=digit.find(&line[sub_start+1..sub_end]);
                        let mut tabstop=((line[digit_indices.start()..digit_indices.end()]).parse::<i32>)+offset;//turbofish
                        tabstop_count+=1;
                        //push everything upto tabstop, followed by new tabstop
                        sub_string.push_str(&line[sub_start..digit_indices.start()]);
                        sub_string.push_str(tabstop.to_string());
                        //sub_string.push_str(&line[digit_indices.end..sub_end]);
                        //TODO: handle embedded tabstops
                        //now sort through the rest of the line
                        start=digit_indices.end();

                    }
                    "@"=>{//if snippet was found

                        let snippet_indices=snippet_finder.find(&line[sub_start+1..]);
                        let sub_snippet_name=&line[snippet_indices.start()..snippet_indices.end()]
                        start=snippet_indices.end()//ex: @if@elif@else
                        if let Some(sub_snippet)=self.snippets.get_mut(&(language.into(),sub_snippet_name).unwrap(){
                            //NOTE: going off the assumption that a snippet should be in brackets
                            //only if it has tab_args
                            if line[indices.start()+1]=="{"{
                                args_indices=snippet_args_finder.find(&line[snippet_indices.end()..])
                                if snippet_indices.end()==args_indices.start(){
                                    snippet_args=(&line[args_indices.start()..args_indices.end()]
                                    start=snippet_args.end()+1;
                                }
                            }
                            assembled_snippet.append(sub_snippet.body);
                        }
                    }
                    _=>{
                        panic!("AAAAHHHHH");//should never happen
                    }
                }//on to next $ or @ in line, if any
            }
            
            //nothing worth mentioning was found

            if sub_string.is_empty(){
                assembled_snippet.push(line.to_string())
            } else {
                assembled_snippet.push(sub_string.push_str(&line[start..])
            }
            line_done=true;
            start=0;
            
        
        }//no more lines
        snippet.body=assembled_snippet;
        snippet.requires_assembly=false;
        return snippet.body.clone()
    }

}
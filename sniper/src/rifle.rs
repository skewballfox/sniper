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

    pub fn load(&mut self,language: &str,snip_set_name: &str, snippet_data: &str){
    
        let temp: Loader = toml::from_str(snippet_data.into()).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.to_string(),snippet_key.to_owned()),snippet.to_owned());
            snippet_set.push(snippet_key.to_owned());
        }
        self.snippet_sets.insert((language.into(),snip_set_name.into()),SnippetSet::new(snippet_set));
        
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
            let mut offset=0;
            let depth=0;
            let round=self.chamber_snippet(&language,
                &snippet_name,
                &mut offset,
                depth,
                "");
            Some(round)
        } else {
            None
        }
    }

    
    fn chamber_snippet(//note this is still not done, but I'm coming back after feedback
        &mut self, 
        language: &str, //the language which is used as half of the key for the snippet
        snippet_name: &str, //the snippet name, which the other half of the key
        offset: &mut i32, // the value that is used to correct the snippet tabstop
        depth: i32, //the current function call depth
        snippet_args: &str,//TODO: the arguments supplied to override tabstops
        ) -> Vec<String> { //NOTE: return value likely to change to Vec<Vec<Strings>> with len = depth
        lazy_static! {
            static ref digit: Regex = Regex::new(r"[[0-9]&&[^a-zA-Z]]+").unwrap();
            //TODO: deal with escaped characters such as \$ in bash
            static ref modification_needed: Regex = Regex::new(r"(\$\{?\d+)|@").unwrap();
            static ref snippet_finder: Regex = Regex::new("[[a-zA-Z0-9/]&&[^@]]+").unwrap();
            static ref snippet_args_finder: Regex = Regex::new(r"\(.*\)}").unwrap();
        }
        if !self.snippets.get(&(language.into(),snippet_name.to_string())).unwrap().requires_assembly {
            return self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().body.clone();
        }
        //why is offset always 0 on recursive calls
        println!("called on this snippet: {:?}",snippet_name);
        println!("original offset: {:?}",offset);
        let mut assembled_snippet: Vec<String>=Vec::new();
        let mut sub_string=String::new();
        let mut start=0;
        let mut end=0;
        let mut tabstop_count=offset.clone();
        let mut sub_snippet_args="";
        let body_to_parse=self.snippets.get(&(language.into(),snippet_name.to_string())).unwrap().body.clone();
        body_to_parse.iter().for_each(|line| {
            println!("tabstop count at top of outer loop: {:?}",tabstop_count);
            start=0;
            sub_string=String::with_capacity(line.len());
            for submatch in modification_needed.find_iter(line){
                println!("tabstop count at start of submatch: {:?}",tabstop_count);
                if submatch.start()>start {
                    sub_string.push_str(&line[start..submatch.start()]);
                }
                start=submatch.start();
                end=submatch.end();
                let  first_char = line[start..end].chars().nth(0).clone();
                //I only care about two things server-side: tabstops and snippets
                match first_char{
                    Some('$')=>{//if placeholder was found
                        println!("tabstop found");
                        println!("sub_match: {:?}",&line[start..end]);
                        println!("full line during submatch: {:?}",line);
                        if *offset==0{
                            
                            sub_string.push_str(&line[start..end]);

                        } else {
                            let digit_indices=digit.find(&line[start..end]).unwrap();
                            println!("digit: {:?}",&line[start+digit_indices.start()..start+digit_indices.end()]);
                            if snippet_args.is_empty(){
                            
                                let mut tabstop=(line[start+digit_indices.start()..start+digit_indices.end()]).parse::<i32>().unwrap();//turbofish
                                tabstop+=*offset;
                                
                                //push everything upto tabstop, followed by new tabstop
                                sub_string.push_str(&line[start..start+digit_indices.start()]);
                                sub_string.push_str(&tabstop.to_string());
                            
                                //TODO: handle embedded tabstops
                            } else {
                                println!("todo");
                            }
                            
                        }
                        tabstop_count+=1;
                        println!("current tabstop count: {:?}",tabstop_count);
                        start=end;

                    }
                    Some('@')=>{//if snippet was found
                        println!("sub snippet found");
                        
                        let snippet_indices=snippet_finder.find(&line[end..]).unwrap();
                        //TODO: find out why the hell start+1 is the actual start of the snippet
                        let sub_snippet_name=&line[end+snippet_indices.start()..end+snippet_indices.end()];
                        println!("{:?}",sub_snippet_name);
                        start=snippet_indices.end();//ex: @if@elif@else
                        if self.snippets.contains_key(&(language.into(),sub_snippet_name.to_string())){
                            //NOTE: going off the assumption that a snippet should be in brackets
                            //only if it has tab_args
                            /*if sub_chars.next()==Some('{'){
                                let args_indices=snippet_args_finder.find(&line[snippet_indices.end()..]).unwrap();
                                if snippet_indices.end()==args_indices.start(){
                                    snippet_args=&line[args_indices.start()..args_indices.end()];
                                    start=args_indices.end();
                                }
                            }*/
                            println!("tabstop count before pass: {:?}",tabstop_count);
                            
                            assembled_snippet.append(&mut self.chamber_snippet(language.into(),
                                sub_snippet_name,
                                &mut tabstop_count,
                                depth+1,
                                snippet_args)
                        );

                            
                            //println!("tabstop count after pass: {:?}", tabstop_count);
                        }
                    }
                    
                    std::option::Option::Some(_) => {
                        println!("should not happen");
                    }
                    std::option::Option::None => {
                        println!("also should not happen");
                    }
                }//on to next $ or @ in line, if any
            }
            
            //nothing worth mentioning was found

            if tabstop_count == 0 && sub_string.is_empty(){
                assembled_snippet.push(line.to_string())
            } else {
                sub_string.push_str(&line[start..]);
                assembled_snippet.push(sub_string.clone());
            }
            start=0;
            
        
        });//no more lines
        println!("tabstop count at end: {:?}",tabstop_count);
        *offset=tabstop_count;
        //println!("{:#?}",assembled_snippet);
        self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().body=assembled_snippet.clone();
        self.snippets.get_mut(&(language.into(),snippet_name.to_string())).unwrap().requires_assembly=false;
        return assembled_snippet
    }

}
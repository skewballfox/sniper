use crate::snippet::{Snippet,SnippetSet,Loader};
use crate::SnippetParser::SnippetParser;

use dashmap::DashMap;

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

///This struct stores snippets, tracks the sets in play, and handles all logic related to snippet management
/// such as removal, and rebuilding the snippet
#[derive(Debug)]
pub(crate) struct Rifle {
    bolt: SnippetParser;
    pub(crate) snippets: DashMap<(String,String),Snippet>,
    pub(crate) snippet_sets: HashMap<(String,String),SnippetSet>,
}

impl Rifle {
    pub fn new() -> Self {
        Self {
            parser: SnippetParser::new(),
            snippets: DashMap::new(),
            snippet_sets: HashMap::new(), 
        }
    }

    pub fn load(&mut self,language: &str,snip_set_name: &str, snippet_data: &str){
    
        let temp: Loader = toml::from_str(&snippet_data).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.to_string(),snippet_key.to_string()),snippet.to_owned());
            snippet_set.push(snippet_key.to_string());
        }
        self.snippet_sets.insert((language.to_string(),snip_set_name.to_string()),SnippetSet::new(snippet_set));
        
    }
    pub fn unload(&mut self, language: &str, snip_set_to_drop: String) {
        for snippet_key in self.snippet_sets[&(language.to_string(),snip_set_to_drop.clone())].contents.iter(){
            self.snippets.remove(&(language.to_string(),snippet_key.to_string()));
        }
        self.snippet_sets.remove(&(language.to_string(),snip_set_to_drop));
    }

    pub fn fire(&mut self, language: &str,snippet_name: &str) -> Option<Vec<String>> { //TODO: probably need to change the output format
        let mut snippet = unwrap_or_return!(self.snippets.get_mut((language.to_string(),snippet_name.to_string())));
        new_body=self.chamber(language,snippet);
        
    }
    fn chamber(&mut self, language: &str, snippet: &mut Snippet) -> Option<Vec<String>> {
        
        assembled_snippet: Vec<String>=Vec::new();
        sub_string="";
        offset=0;
        let result = parser.base.replace_all(string, |cap: &Captures| {
            match &cap[0] {
                "$" => "str",//used for variables and placeholders
                "@" => "er",
                _ => panic!("We should never get here"),
            }
        });

    }

}
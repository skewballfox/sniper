use dashmap::DashMap;
use crate::snippet::{Snippet,SnippetSet,Loader};
struct Rifle {
    pub(crate) snippets: DashMap<(String,String),Snippet>,
    snippet_sets: HashMap<(String,String),SnippetSet>,
}

impl Rifle {
    pub fn new() -> Self {
        Self {
            snippets: DashMap::new(),
            snippet_sets: HashMap::new(), 
        }
    }

    fn load_snippets(&mut self,language: &str,snip_set_name: &str, snippet_data: &str){
    
        let temp: Loader=toml::from_str(&snippet_data).unwrap();
        let mut snippet_set: Vec<String>= Vec::with_capacity(temp.snippets.len());
        for (snippet_key,snippet) in temp.snippets.iter(){

            self.snippets.insert((language.to_string(),snippet_key.to_string()),snippet.to_owned());
            snippet_set.push(snippet_key.to_string());
        }
        self.snippet_sets.insert((language.to_string(),snip_set_name.to_string()),SnippetSet::new(snippet_set));
        
    }
    fn drop_snippets(&mut self, language: &str, snip_set_to_drop: String) {
        for snippet_key in self.snippet_sets[&(language.to_string(),snip_set_to_drop.clone())].contents.iter(){
            self.snippets.remove(&(language.to_string(),snippet_key.to_string()));
        }
        self.snippet_sets.remove(&(language.to_string(),snip_set_to_drop));
    }

}
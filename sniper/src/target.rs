use dashmap::{DashMap,DashSet};
use std::collections::{HashMap};
use std::hash::{Hash,Hasher};
//NOTE: may not be necessary, may wind up getting rid of this
/*
#[derive(Debug)]
pub struct TargetSession {
    editor_session_id: String,
    targets: HashMap<String,Target>,
}*/
#[derive(Debug)]
pub struct Target {
    session_id: String,
    uri: String,
    language: String,
    //should these go here?
    snippet_triggers: DashMap<String,String>,
    disabled_snippets: DashSet<String>,
    //NOTE: probably unnecessary to track this here, given SnippetSets tracks dependant targets
    //snippet_sets: Vec<String>,
}
/*
impl TargetSession {
    pub fn new(session_id: &str) -> Self {
        Self {
            editor_session_id: editor_session_id.to_string(),
            targets: HashMap::new(),
        }
    }
    pub fn add_target(&mut self, uri: &str,language: &str) {
        self.targets.insert(uri.to_string(),Target::new(uri,language));
    }
}*/

impl Target {
    pub fn new(editor_session_id: &str,uri: &str,language: &str) -> Self {
        Self {
            session_id: editor_session_id.to_string(),
            uri: uri.to_string(),
            language: language.to_string(),
            //should these go here?
            snippet_triggers: DashMap::new(),
            disabled_snippets: DashSet::new(),
        }
    }
    pub fn get_language(self)->String{
        return self.language.clone();
    }

}
impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.session_id==other.session_id && self.uri == other.uri
    }
}
impl Eq for Target {}
impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.session_id.hash(state);
        self.uri.hash(state);
    }
}
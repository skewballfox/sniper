/*
   This is the structs representing the files/editors being tracked and what
   we need to know about them if we don't want to just store the snippets in
   memory for perpetuity.

   Each Target is associated with a set of snippets, a language (or maybe a
   set of languages) and
*/

use crate::util::sniper_proto::SnippetInfo;
use dashmap::DashSet;
use qp_trie::Trie;

//use std::hash::{Hash,Hasher};
//NOTE: may not be necessary, may wind up getting rid of this
/*
#[derive(Debug)]
pub struct TargetManager {
    editor_session_id: String,
    targets: HashMap<String,Target>,
}*/
#[derive(Debug)]
pub struct TargetData {
    pub(crate) language: String,
    //should these go here?
    pub(crate) loaded_snippets: DashSet<String>,
    //disabled_snippets: DashSet<String>,
    //NOTE: probably unnecessary to track this here, given SnippetSets tracks dependant targets
    pub(crate) triggers: Trie<Vec<u8>, SnippetInfo>,
}

impl TargetData {
    pub fn new(language: &str) -> Self {
        Self {
            language: language.to_string(),
            loaded_snippets: DashSet::new(),
            triggers: Trie::new(),
            //should these go here?
            //snippet_triggers: DashMap::new(),
            //disabled_snippets: DashSet::new(),
        }
    }
    pub fn get_language(self) -> String {
        return self.language.clone();
    }
}

/*
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
}*/

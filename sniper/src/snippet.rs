
use std::collections::HashSet;
use std::rc;
use serde::Deserialize;
//these are the currently (planned) supported actions for snippets
#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "action", content = "args")]
pub enum Actions {
    Load(Vec<String>),
    Enable(Vec<String>),
    Disable(Vec<String>),
    //Commands, //potentially script running of commands such as making or renaming a file
    //Adjust, //shouldn't be in file, there to tell sniper to reparse at snippet launch time
}
#[derive(Deserialize, Clone, Debug)]
#[serde(rename = "type")]
pub enum SnippetTypes {
    Shorthand,
    Statement,
    Expression,
    Template,
}


//TODO: consider implementing snippet as a type rather than a struct
// would be combined with a match at runtime to execute appropriate behavior
#[derive(Deserialize, Clone, Debug)]
pub struct Snippet {
    name: String,
    #[serde(rename = "type", default = "default_snippet_type")]
    snippet_type: SnippetTypes,
    body: Vec<String>,
    description: String,
    #[serde(default = "unconditional")]
    is_conditional: bool,
    #[serde(default = "no_action")]
    actions: Vec<Actions>,
}

fn default_snippet_type() -> SnippetTypes {
    SnippetTypes::Shorthand
}

fn unconditional() -> bool {
    false
}

fn no_action() -> Vec<Actions> {
    Vec::new()
}

#[derive(Deserialize, Clone, Debug)]
pub struct Loader {
    #[serde(flatten, with = "tuple_vec_map")]
    pub(crate) snippets: Vec<(String, Snippet)>,
}

/// tracks the set each group of snippets belong to, as well as
/// which targets require them
#[derive(Debug)]
pub struct SnippetSet {
    
    pub(crate) contents: Vec<String>,
    target_counter: i32,
}

impl SnippetSet {
    pub(crate) fn new(contents: Vec<String>)->Self {
        Self {
            contents,
            target_counter: 1,
        }
    }
    pub fn added_target(&mut self){
        self.target_counter+=1;
    }

    pub fn decrement_target(&mut self)->bool{
        if self.target_counter>1{
            self.target_counter-=1;
            false
        } else {
            true
        }
    }

}

use serde::Deserialize;
//these are the currently (planned) supported actions for snippets
#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "action", content = "args")]
pub enum Actions {
    Load(Vec<String>),
    Enable(Vec<String>),
    Disable(Vec<String>),
    //Commands, //potentially script running of commands such as making or renaming a file
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
    prefix: String,
    #[serde(rename = "type", default = "default_snippet_type")]
    snippet_type: SnippetTypes,
    pub(crate) body: Vec<String>,
    description: String,
    #[serde(default = "unconditional")]
    is_conditional: bool,
    #[serde(default = "no_action")]
    actions: Vec<Actions>,
    #[serde(default = "assembly_required")]
    pub(crate) requires_assembly:bool,
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
fn assembly_required() -> bool {
    true
}

#[derive(Deserialize, Clone, Debug)]
pub struct Loader {
    #[serde(flatten, with = "tuple_vec_map")]
    pub(crate) snippets: Vec<(String, Snippet)>,
}

/// tracks the set each group of snippets belong to, as well as
/// how many targets require them, used to determine whether it's necessary
/// to add or drop a group of snippets
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

    pub fn increment_target_count(&mut self){
        self.target_counter+=1;
    }

    ///this is ran when a dropping a target, the returned boolean is used to
    /// determine if the snippets associated with this snippet set need to be 
    /// dropped
    pub fn decrement_target_count(&mut self)->bool{
        if self.target_counter>1{
            self.target_counter-=1;
            false
        } else {
            true
        }
    }

}